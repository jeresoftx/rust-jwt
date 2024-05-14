use mongodb::{bson::oid::ObjectId, bson::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserEntity {
    pub _id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime>,
}

impl UserEntity {
    pub fn new(
        first_name: String,
        last_name: String,
        username: String,
        email: String,
        password: String,
    ) -> Self {
        UserEntity {
            _id: ObjectId::new(),
            first_name,
            last_name,
            username,
            email,
            password,
            created_at:bson::DateTime::now(),
            updated_at:bson::DateTime::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub first_name: String,
    pub last_name: String,
}
