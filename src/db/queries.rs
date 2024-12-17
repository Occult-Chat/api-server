use std::str::FromStr;

use sqlx::{MySql, Pool};
use uuid::Uuid;
use rand;
use super::super::api::*;
use chrono::{DateTime, Utc};

// Auth & User Management
pub async fn register_user(
    pool: &Pool<MySql>,
    username: &str,
    email: &str,
    hashed_password: &str,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)",
        id, username, email, hashed_password
    )
    .execute(pool)
    .await?;
    Ok(id)
}


pub async fn login_user(
    pool: &Pool<MySql>,
    email: &str,
    password: &str,
) -> Result<Option<User>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct UserRow {
        id: Vec<u8>,
        username: String,
        display_name: Option<String>,
        email: String,
        avatar: Option<String>,
        status: String,  // MySQL ENUM comes as String
        custom_status: Option<String>,
        created_at: DateTime<Utc>,
    }

    let user_row = sqlx::query_as!(
        UserRow,
        r#"SELECT 
            id,
            username,
            display_name,
            email,
            avatar,
            status as "status: String",
            custom_status,
            created_at
        FROM users 
        WHERE email = ? AND password_hash = ?"#,
        email, password
    )
    .fetch_optional(pool)
    .await?;

    Ok(user_row.map(|row| User {
        id: Uuid::from_slice(&row.id).unwrap(),
        username: row.username,
        display_name: row.display_name,
        email: row.email,
        avatar: row.avatar,
        status: UserStatus::from_str(&row.status).unwrap_or(UserStatus::Offline),
        custom_status: row.custom_status,
        created_at: row.created_at,
        updated_at: Utc::now(),
    }))
}

pub async fn update_user_status(
    pool: &Pool<MySql>,
    user_id: Uuid,
    status: UserStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET status = ? WHERE id = ?",
        status.to_string(),
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Server Management
pub async fn create_server(
    pool: &Pool<MySql>,
    name: &str,
    owner_id: Uuid,
    description: Option<&str>,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO servers (id, name, owner_id, description) VALUES (?, ?, ?, ?)",
        id, name, owner_id, description
    )
    .execute(pool)
    .await?;
    
    // Create default general channel
    create_channel(pool, id, "general", ChannelType::Text, None, false).await?;
    Ok(id)
}

pub async fn join_server(
    pool: &Pool<MySql>,
    server_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO server_members (server_id, user_id) VALUES (?, ?)",
        server_id, user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn leave_server(
    pool: &Pool<MySql>,
    server_id: Uuid,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM server_members WHERE server_id = ? AND user_id = ?",
        server_id, user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Channel Management
pub async fn create_channel(
    pool: &Pool<MySql>,
    server_id: Uuid,
    name: &str,
    channel_type: ChannelType,
    topic: Option<&str>,
    is_private: bool,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO channels (id, server_id, name, channel_type, topic, is_private) 
         VALUES (?, ?, ?, ?, ?, ?)",
        id, server_id, name, channel_type.to_string(), topic, is_private
    )
    .execute(pool)
    .await?;
    Ok(id)
}

pub async fn get_channel_messages(
    pool: &Pool<MySql>,
    channel_id: Uuid,
    limit: i32,
    before_id: Option<Uuid>,
) -> Result<Vec<Message>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct RawMessage {
        id: Vec<u8>,
        content: String,
        author_id: Vec<u8>,
        channel_id: Vec<u8>,
        created_at: DateTime<Utc>,
        edited_at: Option<DateTime<Utc>>,
        reply_to_id: Option<Vec<u8>>,
        is_pinned: bool,
    }

    let channel_id_bytes = channel_id.as_bytes().to_vec();
    
    let raw_messages = if let Some(before) = before_id {
        let before_bytes = before.as_bytes().to_vec();
        sqlx::query_as!(
            RawMessage,
            r#"
            SELECT id, is_pinned as "is_pinned: bool", reply_to_id, content, author_id, channel_id, created_at, edited_at
            FROM messages 
            WHERE channel_id = ? AND id < ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
            channel_id_bytes,
            before_bytes,
            limit
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            RawMessage,
            r#"
            SELECT id, is_pinned as "is_pinned: bool", reply_to_id, content, author_id, channel_id, created_at, edited_at
            FROM messages 
            WHERE channel_id = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#,
            channel_id_bytes,
            limit
        )
        .fetch_all(pool)
        .await?
    };

    // Convert raw messages to Message type
    let messages = raw_messages
        .into_iter()
        .filter_map(|raw| {
            let id = Uuid::from_slice(&raw.id).ok()?;
            let author_id = Uuid::from_slice(&raw.author_id).ok()?;
            let channel_id = Uuid::from_slice(&raw.channel_id).ok()?;
            let reply_to_id = raw.reply_to_id.as_deref().map(Uuid::from_slice).transpose().ok().flatten();
            let is_pinned = raw.is_pinned;
            
            Some(Message {
                id,
                content: raw.content,
                author_id,
                channel_id,
                is_pinned: raw.is_pinned,
                reply_to_id: raw.reply_to_id.as_deref().map(Uuid::from_slice).transpose().ok().flatten(),
                created_at: raw.created_at,
                edited_at: raw.edited_at,
                attachments: vec![],
                mentions: vec![],
                reactions: vec![],
            })
        })
        .collect();

    Ok(messages)
}

// Message Management
pub async fn create_message(
    pool: &Pool<MySql>,
    channel_id: Uuid,
    author_id: Uuid,
    content: &str,
    reply_to_id: Option<Uuid>,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "INSERT INTO messages (id, channel_id, author_id, content, reply_to_id) 
         VALUES (?, ?, ?, ?, ?)",
        id, channel_id, author_id, content, reply_to_id
    )
    .execute(&mut *tx)
    .await?;

    // Update last_message_id in channel
    sqlx::query!(
        "UPDATE channels SET last_message_id = ? WHERE id = ?",
        id, channel_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(id)
}

pub async fn edit_message(
    pool: &Pool<MySql>,
    message_id: Uuid,
    new_content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE messages SET content = ?, edited_at = CURRENT_TIMESTAMP 
         WHERE id = ?",
        new_content, message_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_message(
    pool: &Pool<MySql>,
    message_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM messages WHERE id = ?", message_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Reactions
pub async fn add_reaction(
    pool: &Pool<MySql>,
    message_id: Uuid,
    user_id: Uuid,
    emoji: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO reactions (message_id, user_id, emoji) VALUES (?, ?, ?)",
        message_id, user_id, emoji
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_reaction(
    pool: &Pool<MySql>,
    message_id: Uuid,
    user_id: Uuid,
    emoji: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM reactions 
         WHERE message_id = ? AND user_id = ? AND emoji = ?",
        message_id, user_id, emoji
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Attachments
pub async fn add_attachment(
    pool: &Pool<MySql>,
    message_id: Uuid,
    file_type: AttachmentType,
    url: &str,
    filename: &str,
    size: i64,
    mime_type: &str,
) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO attachments 
         (id, message_id, attachment_type, url, filename, size, mime_type) 
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        id, message_id, file_type.to_string(), url, filename, size, mime_type
    )
    .execute(pool)
    .await?;
    Ok(id)
}

// Invites
pub async fn create_invite(
    pool: &Pool<MySql>,
    server_id: Uuid,
    inviter_id: Uuid,
    max_uses: Option<i32>,
    expires_in: Option<i32>,
) -> Result<String, sqlx::Error> {
    let code = generate_invite_code();
    let expires_at = expires_in.map(|hours| {
        chrono::Utc::now() + chrono::Duration::hours(hours as i64)
    });

    sqlx::query!(
        "INSERT INTO invites (code, server_id, inviter_id, max_uses, expires_at) 
         VALUES (?, ?, ?, ?, ?)",
        code, server_id, inviter_id, max_uses, expires_at
    )
    .execute(pool)
    .await?;
    Ok(code)
}

pub async fn use_invite(
    pool: &Pool<MySql>,
    code: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let invite = sqlx::query!(
        "SELECT server_id, uses, max_uses, expires_at 
         FROM invites 
         WHERE code = ? 
         AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP)
         AND (max_uses IS NULL OR uses < max_uses)",
        code
    )
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(invite) = invite {
        sqlx::query!(
            "UPDATE invites SET uses = uses + 1 WHERE code = ?",
            code
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(Some(Uuid::from_slice(&invite.server_id).map_err(|_| sqlx::Error::Decode("Invalid UUID".into()))?))
    } else {
        Ok(None)
    }
}

// Helper function to generate random invite code
fn generate_invite_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const CODE_LEN: usize = 8;
    
    let mut rng = rand::thread_rng();
    
    (0..CODE_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}