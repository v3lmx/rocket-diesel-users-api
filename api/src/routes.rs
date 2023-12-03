use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::db::pool::UsersDb;
use crate::db::UserDao;
use crate::prelude::ApiError;
use crate::user::{NewUser, UpdateUser, User};

#[get("/user")]
pub async fn get_users(mut db: Connection<UsersDb>) -> Result<Json<Vec<User>>, ApiError> {
    let users = db.get_all().await?;
    Ok(Json(users))
}

#[get("/user/<uuid>")]
pub async fn get_user(uuid: &str, mut db: Connection<UsersDb>) -> Result<Json<User>, ApiError> {
    let uuid = Uuid::parse_str(uuid)?;
    let user = db.get(uuid).await?;

    println!("user: {:?}", user);
    Ok(Json(user))
}

#[post("/user", data = "<user>")]
pub async fn create_user(
    user: Json<NewUser>,
    mut db: Connection<UsersDb>,
) -> Result<Status, ApiError> {
    db.create(user.into_inner()).await?;
    Ok(Status::Created)
}

#[put("/user/<uuid>", data = "<user>")]
pub async fn update_user(
    uuid: &str,
    user: Json<UpdateUser>,
    mut db: Connection<UsersDb>,
) -> Result<(), ApiError> {
    let uuid = Uuid::parse_str(uuid).map_err(|_| ApiError::Server("Invalid UUID".into()))?;
    db.update(uuid, user.into_inner()).await?;
    Ok(())
}

#[delete("/user/<uuid>")]
pub async fn delete_user(uuid: &str, mut db: Connection<UsersDb>) -> Result<(), ApiError> {
    let uuid = Uuid::parse_str(uuid).map_err(|_| ApiError::Server("Invalid UUID".into()))?;
    db.delete(uuid).await?;
    Ok(())
}
