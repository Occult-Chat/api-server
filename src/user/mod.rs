use uuid::Uuid;
use url::Url;
pub mod auth;
enum UserStatus {
    
}
struct User {
    uuid: Uuid,
    username: String,
    email: String,
    status: UserStatus,
    status_message: Option<String>,
    avatar_url: Url,
}