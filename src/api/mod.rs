use anyhow::{anyhow, Result};
use log::{error, info};
use rocket::{delete, get, patch, post, put, routes, FromFormField, State};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use serde::{Serialize, Deserialize};
use url::Url;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::str::FromStr;

// Models
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub avatar: Option<String>,
    pub status: UserStatus,
    pub custom_status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromFormField)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub server_id: Uuid,
    pub topic: Option<String>,
    pub slow_mode: Option<i32>,
    pub is_private: bool,
    pub last_message_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChannelType {
    Text,
    Voice,
    Announcement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub channel_id: Uuid,
    pub is_pinned: bool,
    pub reply_to_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub attachments: Vec<Attachment>,
    pub mentions: Vec<User>,
    pub reactions: Vec<Reaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub attachment_type: AttachmentType,
    pub url: String,
    pub filename: String,
    pub size: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentType {
    Image,
    Video,
    File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub emoji: String,
    pub count: i32,
    pub has_reacted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Request/Response structs
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct CreateServerForm<'r> {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<TempFile<'r>>,
}

#[derive(Debug, FromForm)]
pub struct UpdateServerForm<'r> {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<TempFile<'r>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub topic: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChannelRequest {
    pub name: Option<String>,
    pub topic: Option<String>,
    pub slow_mode: Option<i32>,
}

#[derive(Debug, FromForm)]
pub struct CreateMessageForm<'r> {
    pub content: String,
    pub reply_to_id: Option<String>,
    pub attachments: Vec<TempFile<'r>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: String,
}

#[derive(Debug, FromForm)]
pub struct UpdateUserForm<'r> {
    pub username: Option<String>,
    pub avatar: Option<TempFile<'r>>,
    pub status: Option<UserStatus>,
    pub custom_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinServerRequest {
    pub invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    pub max_uses: Option<i32>,
    pub expires_in: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteResponse {
    pub code: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_uses: Option<i32>,
    pub uses: i32,
}

impl FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "online" => Ok(UserStatus::Online),
            "idle" => Ok(UserStatus::Idle),
            "dnd" => Ok(UserStatus::Dnd),
            "offline" => Ok(UserStatus::Offline),
            _ => Err(format!("Invalid user status: {}", s)),
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Online => write!(f, "online"),
            UserStatus::Idle => write!(f, "idle"),
            UserStatus::Dnd => write!(f, "dnd"),
            UserStatus::Offline => write!(f, "offline"),
        }
    }
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::Text => write!(f, "text"),
            ChannelType::Voice => write!(f, "voice"),
            ChannelType::Announcement => write!(f, "announcement"),
        }
    }
}

impl std::fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachmentType::Image => write!(f, "image"),
            AttachmentType::Video => write!(f, "video"),
            AttachmentType::File => write!(f, "file"),
        }
    }
}



// Auth Routes
#[post("/auth/login", format = "json", data = "<login>")]
async fn login(login: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    info!("Login attempt for user: {:?}", login.username.as_ref().or(login.email.as_ref()));
    Err(Status::NotImplemented)
}

#[post("/auth/register", format = "json", data = "<registration>")]
async fn register(registration: Json<RegisterRequest>) -> Result<Json<User>, Status> {
    info!("Registration attempt for user: {}", registration.username);
    Err(Status::NotImplemented)
}

// Server Routes
#[get("/servers")]
async fn get_servers() -> Result<Json<Vec<Server>>, Status> {
    info!("Fetching servers for user");
    Err(Status::NotImplemented)
}

#[post("/servers", data = "<form>")]
async fn create_server(form: Form<CreateServerForm<'_>>) -> Result<Json<Server>, Status> {
    info!("Creating new server: {}", form.name);
    Err(Status::NotImplemented)
}

#[get("/servers/<server_id>")]
async fn get_server(server_id: String) -> Result<Json<Server>, Status> {
    info!("Fetching server: {}", server_id);
    Err(Status::NotImplemented)
}

#[patch("/servers/<server_id>", data = "<form>")]
async fn update_server(server_id: String, form: Form<UpdateServerForm<'_>>) -> Result<Json<Server>, Status> {
    info!("Updating server: {}", server_id);
    Err(Status::NotImplemented)
}

#[delete("/servers/<server_id>")]
async fn delete_server(server_id: String) -> Status {
    info!("Deleting server: {}", server_id);
    Status::NoContent
}

// Channel Routes
#[get("/servers/<server_id>/channels")]
async fn get_channels(server_id: String) -> Result<Json<Vec<Channel>>, Status> {
    info!("Fetching channels for server: {}", server_id);
    Err(Status::NotImplemented)
}

#[post("/servers/<server_id>/channels", format = "json", data = "<channel>")]
async fn create_channel(server_id: String, channel: Json<CreateChannelRequest>) -> Result<Json<Channel>, Status> {
    info!("Creating channel in server {}: {}", server_id, channel.name);
    Err(Status::NotImplemented)
}

#[get("/channels/<channel_id>")]
async fn get_channel(channel_id: String) -> Result<Json<Channel>, Status> {
    info!("Fetching channel: {}", channel_id);
    Err(Status::NotImplemented)
}

#[patch("/channels/<channel_id>", format = "json", data = "<channel>")]
async fn update_channel(channel_id: String, channel: Json<UpdateChannelRequest>) -> Result<Json<Channel>, Status> {
    info!("Updating channel: {}", channel_id);
    Err(Status::NotImplemented)
}

#[delete("/channels/<channel_id>")]
async fn delete_channel(channel_id: String) -> Status {
    info!("Deleting channel: {}", channel_id);
    Status::NoContent
}

// Message Routes
#[get("/channels/<channel_id>/messages?<before>&<after>&<limit>")]
async fn get_messages(
    channel_id: String,
    before: Option<String>,
    after: Option<String>,
    limit: Option<i32>,
) -> Result<Json<Vec<Message>>, Status> {
    info!("Fetching messages for channel: {}", channel_id);
    Err(Status::NotImplemented)
}

#[post("/servers/<server_id>/channels/<channel_id>/messages", data = "<form>")]
async fn create_message(user: AuthenticatedUser, channel_id: String, server_id: String, form: Form<CreateMessageForm<'_>>) -> Result<Status, String> {
    info!("Creating message in channel: {}", channel_id);
    info!("Message contents: {}", form.content);
    user.user_id;




    Err(format!("{}",form.content))
}

#[patch("/channels/<channel_id>/messages/<message_id>", format = "json", data = "<message>")]
async fn update_message(
    channel_id: String,
    message_id: String,
    message: Json<UpdateMessageRequest>,
) -> Result<Json<Message>, Status> {
    info!("Updating message {} in channel {}", message_id, channel_id);
    Err(Status::NotImplemented)
}

#[delete("/channels/<channel_id>/messages/<message_id>")]
async fn delete_message(channel_id: String, message_id: String) -> Status {
    info!("Deleting message {} from channel {}", message_id, channel_id);
    Status::NoContent
}

// Pin Routes
#[put("/channels/<channel_id>/pins/<message_id>")]
async fn pin_message(channel_id: String, message_id: String) -> Status {
    info!("Pinning message {} in channel {}", message_id, channel_id);
    Status::NoContent
}

#[delete("/channels/<channel_id>/pins/<message_id>")]
async fn unpin_message(channel_id: String, message_id: String) -> Status {
    info!("Unpinning message {} in channel {}", message_id, channel_id);
    Status::NoContent
}

// Typing Indicator Route
#[post("/channels/<channel_id>/typing")]
async fn send_typing_indicator(channel_id: String) -> Status {
    info!("Sending typing indicator in channel: {}", channel_id);
    Status::NoContent
}

// Reaction Routes
#[put("/channels/<channel_id>/reactions/<message_id>/<emoji>")]
async fn add_reaction(channel_id: String, message_id: String, emoji: String) -> Status {
    info!("Adding reaction {} to message {} in channel {}", emoji, message_id, channel_id);
    Status::NoContent
}

#[delete("/channels/<channel_id>/reactions/<message_id>/<emoji>")]
async fn remove_reaction(channel_id: String, message_id: String, emoji: String) -> Status {
    info!("Removing reaction {} from message {} in channel {}", emoji, message_id, channel_id);
    Status::NoContent
}

// User Routes
#[get("/users/@me")]
async fn get_current_user() -> Result<Json<User>, Status> {
    info!("Fetching current user profile");
    Err(Status::NotImplemented)
}

#[patch("/users/@me", data = "<form>")]
async fn update_current_user(form: Form<UpdateUserForm<'_>>) -> Result<Json<User>, Status> {
    info!("Updating current user profile");
    Err(Status::NotImplemented)
}

#[get("/users/<user_id>")]
async fn get_user(user_id: String) -> Result<Json<User>, Status> {
    info!("Fetching user profile: {}", user_id);
    Err(Status::NotImplemented)
}

// Server Join/Invite Routes
#[post("/users/@me/servers", format = "json", data = "<join_request>")]
async fn join_server(join_request: Json<JoinServerRequest>) -> Result<Json<Server>, Status> {
    info!("Joining server with invite code: {}", join_request.invite_code);
    Err(Status::NotImplemented)
}

#[post("/servers/<server_id>/invites", format = "json", data = "<invite_request>")]
async fn create_invite(
    server_id: String,
    invite_request: Json<CreateInviteRequest>,
) -> Result<Json<InviteResponse>, Status> {
    info!("Creating invite for server: {}", server_id);
    Err(Status::NotImplemented)
}

// Attachment Routes
#[post("/channels/<channel_id>/attachments", data = "<form>")]
async fn upload_attachments(
    channel_id: String,
    form: Form<Vec<TempFile<'_>>>,
) -> Result<Json<Vec<Attachment>>, Status> {
    info!("Uploading attachments to channel: {}", channel_id);
    Err(Status::NotImplemented)
}

pub async fn start_listener(config: &ServerConfig) -> Result<()> {
    let log_level = &config.log_level.as_str().to_lowercase();
    println!("Starting occult server. Current log level: {log_level}");
    log::set_max_level(config.log_level);

    let _server = rocket::build()
        .configure(rocket::Config {
            port: config.http_port.get() as u16,
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .mount("/", routes![
            // Auth routes
            login,
            register,
            // Server routes
            get_servers,
            create_server,
            get_server,
            update_server,
            delete_server,
            // Channel routes
            get_channels,
            create_channel,
            get_channel,
            update_channel,
            delete_channel,
            // Message routes
            get_messages,
            create_message,
            update_message,
            delete_message,
            // Pin routes
            pin_message,
            unpin_message,
            // Typing indicator
            send_typing_indicator,
            // Reaction routes
            add_reaction,
            remove_reaction,
            // User routes
            get_current_user,
            update_current_user,
            get_user,
            // Server join/invite routes
            join_server,
            create_invite,
            // Attachment routes
            upload_attachments,
        ])
        .launch()
        .await
        .map_err(|e| {
            error!("An error occurred. {e}");
            anyhow!(format!("Failed to start rocket server: {e:#}"))
        });

    Ok(())
}


// Error handling implementation
#[rocket::catch(404)]
fn not_found() -> Json<Error> {
    Json(Error {
        code: "NOT_FOUND".to_string(),
        message: "The requested resource was not found".to_string(),
        details: None,
    })
}

#[rocket::catch(401)]
fn unauthorized() -> Json<Error> {
    Json(Error {
        code: "UNAUTHORIZED".to_string(),
        message: "Authentication is required to access this resource".to_string(),
        details: None,
    })
}

#[rocket::catch(403)]
fn forbidden() -> Json<Error> {
    Json(Error {
        code: "FORBIDDEN".to_string(),
        message: "You don't have permission to access this resource".to_string(),
        details: None,
    })
}

#[rocket::catch(500)]
fn internal_error() -> Json<Error> {
    Json(Error {
        code: "INTERNAL_SERVER_ERROR".to_string(),
        message: "An internal server error occurred".to_string(),
        details: None,
    })
}

// JWT Authentication Guard implementation
use rocket::request::{FromRequest, Outcome, Request};

use crate::workspace::ServerConfig;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the authorization header
        let auth_header = request.headers().get_one("Authorization");
        
        match auth_header {
            Some(header) => {
                if !header.starts_with("Bearer ") {
                    return Outcome::Error((
                        Status::Unauthorized,
                        Error {
                            code: "INVALID_TOKEN".to_string(),
                            message: "Invalid authentication token format".to_string(),
                            details: None,
                        },
                    ));
                }

                let token = header.replace("Bearer ", "");
                
                // TODO: Implement JWT validation
                // For now, return a mock user
                Outcome::Success(AuthenticatedUser {
                    user_id: Uuid::new_v4(),
                })
            }
            None => Outcome::Error((
                Status::Unauthorized,
                Error {
                    code: "MISSING_TOKEN".to_string(),
                    message: "Authentication token is required".to_string(),
                    details: None,
                },
            )),
        }
    }
}

// Helper functions for common operations
async fn validate_server_access(user_id: Uuid, server_id: Uuid) -> Result<bool, Status> {
    // TODO: Implement server access validation
    Ok(true)
}

async fn validate_channel_access(user_id: Uuid, channel_id: Uuid) -> Result<bool, Status> {
    // TODO: Implement channel access validation
    Ok(true)
}

async fn validate_message_ownership(user_id: Uuid, message_id: Uuid) -> Result<bool, Status> {
    // TODO: Implement message ownership validation
    Ok(true)
}

// File handling utilities
async fn save_file(file: TempFile<'_>, path: &str) -> Result<String, Status> {
    // TODO: Implement file saving logic
    Ok("file_url".to_string())
}

async fn validate_file(file: &TempFile<'_>) -> Result<(), Status> {
    // TODO: Implement file validation logic
    Ok(())
}