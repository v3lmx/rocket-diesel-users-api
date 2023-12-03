use rocket_db_pools::{
    diesel::{prelude::*, PgPool},
    Connection, Database,
};
use uuid::Uuid;

use crate::db::{
    models::{DbUser, NewDbUser, UpdateDbUser},
    schema,
};
use crate::prelude::*;
use crate::user::{NewUser, UpdateUser, User};

use super::UserDao;

/// Pool used by rocket to manage database connections.
#[derive(Database)]
#[database("user_db")]
pub struct UsersDb(PgPool);

#[async_trait]
impl UserDao for Connection<UsersDb> {
    async fn get_all(&mut self) -> Result<Vec<User>> {
        use schema::users;

        let users = users::table
            .load::<DbUser>(self)
            .await
            .map_err(|err| ApiError::Server(format!("Failed to get users: {}", err)))?;

        let users = users
            .into_iter()
            .map(|user| user.try_into())
            .collect::<Result<Vec<User>>>()?;

        Ok(users)
    }

    async fn get(&mut self, uuid: Uuid) -> Result<User> {
        use schema::users;

        let user = users::table
            .filter(users::uuid.eq(uuid))
            .first::<DbUser>(self)
            .await
            .optional();

        println!("user: {:?}", user);

        match user {
            Ok(Some(user)) => Ok(user.try_into()?),
            Ok(None) => Err(ApiError::NotFound("User not found.".into())),
            Err(err) => Err(ApiError::Server(format!("Failed to get user: {}", err))),
        }
    }

    async fn create(&mut self, new_user: NewUser) -> Result<()> {
        use schema::users;

        let new_db_user: NewDbUser = new_user.try_into()?;

        let res = diesel::insert_into(users::table)
            .values(&new_db_user)
            .execute(self)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(ApiError::Server(format!("Failed to save user: {}", err))),
        }
    }

    async fn update(&mut self, uuid: Uuid, update_user: UpdateUser) -> Result<()> {
        use schema::users;

        let update_user: UpdateDbUser = update_user.try_into()?;

        let res = diesel::update(users::table.filter(users::uuid.eq(uuid)))
            .set(&update_user)
            .execute(self)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(ApiError::Server(format!("Failed to update user: {}", err))),
        }
    }

    async fn delete(&mut self, uuid: Uuid) -> Result<()> {
        use schema::users;

        let res = diesel::delete(users::table.filter(users::uuid.eq(uuid)))
            .execute(self)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(ApiError::Server(format!("Failed to delete user: {}", err))),
        }
    }
}
