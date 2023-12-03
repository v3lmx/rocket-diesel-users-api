use uuid::Uuid;

use crate::prelude::*;
use crate::user::{NewUser, UpdateUser, User};

mod models;
pub mod pool;
mod schema;

/// The trait that defines the database operations, and has to be
/// implemented on the database mechanism (Connection in case of DB)
#[async_trait]
pub trait UserDao {
    async fn get_all(&mut self) -> Result<Vec<User>>;
    async fn get(&mut self, uuid: Uuid) -> Result<User>;
    async fn create(&mut self, new_user: NewUser) -> Result<()>;
    async fn update(&mut self, uuid: Uuid, user: UpdateUser) -> Result<()>;
    async fn delete(&mut self, uuid: Uuid) -> Result<()>;
}
