use std::str::FromStr;

use chrono::NaiveDateTime;
use email_address::EmailAddress;
use rocket_db_pools::diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    prelude::{ApiError, Result},
    user::{NewUser, Role, UpdateUser, User},
};

use super::schema;

#[derive(Clone, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
pub struct DbUser {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewDbUser {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UpdateDbUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
}

impl TryFrom<DbUser> for User {
    type Error = ApiError;

    fn try_from(user: DbUser) -> Result<Self> {
        let role = match user.role.as_str() {
            "User" => Ok(Role::User),
            "Admin" => Ok(Role::Admin),
            _ => Err(ApiError::Server("Invalid role".into())),
        }?;

        let email = EmailAddress::from_str(user.email.as_str())
            .map_err(|_| ApiError::Server("Invalid email".into()))?;

        let re = regex::Regex::new(r"^\$2[ayb]\$.{56}$").expect("Invalid regex");
        if !re.is_match(user.password_hash.as_str()) {
            return Err(ApiError::Server("Invalid password hash".into()));
        }

        Ok(Self {
            uuid: user.uuid,
            first_name: user.first_name,
            last_name: user.last_name,
            email,
            password_hash: user.password_hash,
            role,
        })
    }
}

impl TryFrom<NewUser> for NewDbUser {
    type Error = ApiError;

    fn try_from(value: NewUser) -> std::result::Result<Self, Self::Error> {
        let role = match value.role {
            Role::User => "User".to_string(),
            Role::Admin => "Admin".to_string(),
        };

        let re = regex::Regex::new(r"^\$2[ayb]\$.{56}$").expect("Invalid regex");
        if !re.is_match(value.password_hash.as_str()) {
            return Err(ApiError::Server("Invalid password hash".into()));
        }

        Ok(Self {
            uuid: Uuid::new_v4(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email.to_string(),
            password_hash: value.password_hash,
            role,
        })
    }
}

impl TryFrom<UpdateUser> for UpdateDbUser {
    type Error = ApiError;

    fn try_from(value: UpdateUser) -> std::result::Result<Self, Self::Error> {
        if value.password_hash.is_some() {
            let re = regex::Regex::new(r"^\$2[ayb]\$.{56}$").expect("Invalid regex");
            if !re.is_match(value.password_hash.as_ref().unwrap().as_str()) {
                return Err(ApiError::Server("Invalid password hash".into()));
            }
        }

        let role = match value.role {
            Some(Role::User) => Some("User".to_string()),
            Some(Role::Admin) => Some("Admin".to_string()),
            None => None,
        };

        Ok(Self {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email.map(|email| email.to_string()),
            password_hash: value.password_hash,
            role,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::user::UpdateUser;

    use super::*;

    fn get_db_user() -> DbUser {
        DbUser {
            uuid: Uuid::new_v4(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@email.com".to_string(),
            password_hash: "$2a$10$WowcJIP00VXf6HhTeeoOrudjFlyVRKqcMWaSWaJV.NoqRpntx0HQC"
                .to_string(),
            role: "User".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }

    fn get_new_user() -> NewUser {
        NewUser {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: EmailAddress::from_str("john.doe@email.com").unwrap(),
            password_hash: "$2a$10$WowcJIP00VXf6HhTeeoOrudjFlyVRKqcMWaSWaJV.NoqRpntx0HQC"
                .to_string(),
            role: Role::User,
        }
    }

    fn get_update_user() -> UpdateUser {
        UpdateUser {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some(EmailAddress::from_str("john.doe@email.com").unwrap()),
            password_hash: Some(
                "$2a$10$WowcJIP00VXf6HhTeeoOrudjFlyVRKqcMWaSWaJV.NoqRpntx0HQC".to_string(),
            ),
            role: Some(Role::User),
        }
    }

    #[test]
    fn test_from_db_user() {
        let db_user = get_db_user();

        let user: User = db_user.clone().try_into().unwrap();

        assert_eq!(user.uuid, db_user.uuid);
        assert_eq!(user.first_name, db_user.first_name);
        assert_eq!(user.last_name, db_user.last_name);
        let db_email = &db_user.email;
        let email = EmailAddress::from_str(db_email).unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, db_user.password_hash);
        assert_eq!(user.role, Role::User);
    }

    #[test]
    fn test_wrong_email() {
        let mut db_user = get_db_user();
        db_user.email = "this is not an email".to_string();

        let res: Result<User> = db_user.try_into();
        assert!(res.is_err());
    }

    #[test]
    fn test_wrong_password_hash() {
        let mut db_user = get_db_user();
        db_user.password_hash = "this is not an hash".to_string();

        let res: Result<User> = db_user.try_into();
        assert!(res.is_err());
    }

    #[test]
    fn test_wrong_role() {
        let mut db_user = get_db_user();
        db_user.email = "this is not a role".to_string();

        let res: Result<User> = db_user.try_into();
        assert!(res.is_err());
    }

    #[test]
    fn test_from_new_user() {
        let new_user = get_new_user();

        let new_db_user: NewDbUser = new_user.clone().try_into().unwrap();
        assert_eq!(new_user.first_name, new_db_user.first_name);
        assert_eq!(new_user.last_name, new_db_user.last_name);
        assert_eq!(new_user.email.to_string(), new_db_user.email);
        assert_eq!(new_user.password_hash, new_db_user.password_hash);
        let role = match new_user.role {
            Role::User => "User".to_string(),
            Role::Admin => "Admin".to_string(),
        };
        assert_eq!(role, new_db_user.role);
    }

    #[test]
    fn test_from_update_user() {
        let update_user = get_update_user();

        let update_db_user: UpdateDbUser = update_user.clone().try_into().unwrap();
        assert_eq!(update_user.first_name, update_db_user.first_name);
        assert_eq!(update_user.last_name, update_db_user.last_name);
        assert_eq!(
            update_user.email.map(|email| email.to_string()),
            update_db_user.email
        );
        assert_eq!(update_user.password_hash, update_db_user.password_hash);
        let role = match update_user.role {
            Some(Role::User) => Some("User".to_string()),
            Some(Role::Admin) => Some("Admin".to_string()),
            None => None,
        };
        assert_eq!(role, update_db_user.role);
    }
}
