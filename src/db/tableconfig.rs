use sqlx::Transaction;
use sqlx::MySql;

pub async fn init_tables(transaction: &mut Transaction<'_, MySql>) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query( 
        "CREATE TABLE IF NOT EXISTS users (
            id BINARY(16) PRIMARY KEY,
            username VARCHAR(32) NOT NULL UNIQUE,
            display_name VARCHAR(32),
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            avatar TEXT,
            status ENUM('online', 'idle', 'dnd', 'offline') NOT NULL DEFAULT 'offline',
            custom_status TEXT,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create servers table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS servers (
            id BINARY(16) PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            description TEXT,
            icon TEXT,
            owner_id BINARY(16) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create channels table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS channels (
            id BINARY(16) PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            channel_type ENUM('text', 'voice', 'announcement') NOT NULL,
            server_id BINARY(16) NOT NULL,
            topic TEXT,
            slow_mode INT,
            is_private BOOLEAN NOT NULL DEFAULT false,
            last_message_id BINARY(16),
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create messages table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id BINARY(16) PRIMARY KEY,
            content TEXT NOT NULL,
            author_id BINARY(16) NOT NULL,
            channel_id BINARY(16) NOT NULL,
            reply_to_id BINARY(16),
            is_pinned BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            edited_at TIMESTAMP NULL,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
            FOREIGN KEY (reply_to_id) REFERENCES messages(id) ON DELETE SET NULL
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Update channels.last_message_id foreign key
    sqlx::query(
        "ALTER TABLE channels 
         ADD FOREIGN KEY (last_message_id) 
         REFERENCES messages(id) 
         ON DELETE SET NULL"
    )
    .execute(&mut **transaction)
    .await?;

    // Create attachments table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS attachments (
            id BINARY(16) PRIMARY KEY,
            message_id BINARY(16) NOT NULL,
            attachment_type ENUM('image', 'video', 'file') NOT NULL,
            url TEXT NOT NULL,
            filename VARCHAR(255) NOT NULL,
            size BIGINT NOT NULL,
            mime_type VARCHAR(127) NOT NULL,
            width INT,
            height INT,
            duration INT,
            FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create reactions table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS reactions (
            message_id BINARY(16) NOT NULL,
            user_id BINARY(16) NOT NULL,
            emoji VARCHAR(32) NOT NULL,
            PRIMARY KEY (message_id, user_id, emoji),
            FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create mentions table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mentions (
            message_id BINARY(16) NOT NULL,
            user_id BINARY(16) NOT NULL,
            PRIMARY KEY (message_id, user_id),
            FOREIGN KEY (message_id) REFERENCES messages(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create server_members table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS server_members (
            user_id BINARY(16) NOT NULL,
            server_id BINARY(16) NOT NULL,
            joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (user_id, server_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    // Create invites table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS invites (
            code VARCHAR(10) PRIMARY KEY,
            server_id BINARY(16) NOT NULL,
            inviter_id BINARY(16) NOT NULL,
            max_uses INT,
            uses INT NOT NULL DEFAULT 0,
            expires_at TIMESTAMP NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE,
            FOREIGN KEY (inviter_id) REFERENCES users(id) ON DELETE CASCADE
        )"
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}