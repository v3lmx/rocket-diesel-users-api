use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: EmailAddress,
    pub password_hash: String,
    pub role: Role,
}

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: EmailAddress,
    pub password_hash: String,
    pub role: Role,
}

#[derive(Clone, Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<EmailAddress>,
    pub password_hash: Option<String>,
    pub role: Option<Role>,
}
