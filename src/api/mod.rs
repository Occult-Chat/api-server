// src/main.rs
use anyhow::{anyhow, Result};
use log::{error, info};
use rocket::{get, post, put, delete, patch, routes, State};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::Form;
use rocket::fs::TempFile;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Models
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    username: String,
    display_name: Option<String>,
    email: String,
    avatar: Option<String>,
    status: UserStatus,
    custom_status: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    id: Uuid,
    name: String,
    description: Option<String>,
    icon: Option<String>,
    owner_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: Uuid,
    name: String,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    server_id: Uuid,
    topic: Option<String>,
    slow_mode: Option<i32>,
    is_private: bool,
    last_message_id: Option<Uuid>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
    id: Uuid,
    content: String,
    author_id: Uuid,
    channel_id: Uuid,
    reply_to_id: Option<Uuid>,
    is_pinned: bool,
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
    attachments: Vec<Attachment>,
    mentions: Vec<User>,
    reactions: Vec<Reaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    id: Uuid,
    #[serde(rename = "type")]
    attachment_type: AttachmentType,
    url: String,
    filename: String,
    size: i64,
    mime_type: String,
    width: Option<i32>,
    height: Option<i32>,
    duration: Option<i32>,
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
    emoji: String,
    count: i32,
    has_reacted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    code: String,
    message: String,
    details: Option<serde_json::Value>,
}

// Request/Response structs
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, FromForm)]
pub struct CreateServerForm<'r> {
    name: String,
    description: Option<String>,
    icon: Option<TempFile<'r>>,
}

#[derive(Debug, FromForm)]
pub struct UpdateServerForm<'r> {
    name: Option<String>,
    description: Option<String>,
    icon: Option<TempFile<'r>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    name: String,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    topic: Option<String>,
    is_private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChannelRequest {
    name: Option<String>,
    topic: Option<String>,
    slow_mode: Option<i32>,
}

#[derive(Debug, FromForm)]
pub struct CreateMessageForm<'r> {
    content: String,
    reply_to_id: Option<String>,
    attachments: Vec<TempFile<'r>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    content: String,
}

#[derive(Debug, FromForm)]
pub struct UpdateUserForm<'r> {
    username: Option<String>,
    avatar: Option<TempFile<'r>>,
    status: Option<UserStatus>,
    custom_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinServerRequest {
    invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    max_uses: Option<i32>,
    expires_in: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteResponse {
    code: String,
    expires_at: Option<DateTime<Utc>>,
    max_uses: Option<i32>,
    uses: i32,
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

#[post("/channels/<channel_id>/messages", data = "<form>")]
async fn create_message(channel_id: String, form: Form<CreateMessageForm<'_>>) -> Result<Json<Message>, Status> {
    info!("Creating message in channel: {}", channel_id);
    Err(Status::NotImplemented)
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

// Server Configuration
pub struct ServerConfig {
    port: u16,
    log_level: log::LevelFilter,
}

pub async fn start_listener(config: &ServerConfig) -> Result<()> {
    let log_level = &config.log_level.as_str().to_lowercase();
    println!("Starting occult server. Current log level: {log_level}");
    log::set_max_level(config.log_level);

    let _server = rocket::build()
        .configure(rocket::Config {
            port: config.port,
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

// Main function implementation
#[rocket::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Create server configuration
    let config = ServerConfig {
        port: 3000,
        log_level: log::LevelFilter::Info,
    };

    // Start the server
    start_listener(&config).await?;

    Ok(())
}

// Error handling implementation
#[catch(404)]
fn not_found() -> Json<Error> {
    Json(Error {
        code: "NOT_FOUND".to_string(),
        message: "The requested resource was not found".to_string(),
        details: None,
    })
}

#[catch(401)]
fn unauthorized() -> Json<Error> {
    Json(Error {
        code: "UNAUTHORIZED".to_string(),
        message: "Authentication is required to access this resource".to_string(),
        details: None,
    })
}

#[catch(403)]
fn forbidden() -> Json<Error> {
    Json(Error {
        code: "FORBIDDEN".to_string(),
        message: "You don't have permission to access this resource".to_string(),
        details: None,
    })
}

#[catch(500)]
fn internal_error() -> Json<Error> {
    Json(Error {
        code: "INTERNAL_SERVER_ERROR".to_string(),
        message: "An internal server error occurred".to_string(),
        details: None,
    })
}

// JWT Authentication Guard implementation
use rocket::request::{FromRequest, Outcome, Request};

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