use crate::auth::check::Auth;

use crate::models::user::{CreateUserSchema, UserModel};

use actix_web::{post, web, web::Data, HttpResponse};
use serde_json::json;

use mongodb::{bson, Client};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use std::env;

fn encrypt_password(password: &str) -> Result<String, String> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|e| e.to_string())?
        .to_string();

    Ok(password_hash)
}

#[post("/user/add")]
async fn add_user(
    client: Data<Client>,
    Auth(_user): Auth,
    body: web::Json<CreateUserSchema>,
) -> HttpResponse {
    let mongo_db = env::var("MONGO_DB").expect("Can't get mongo db name");
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = encrypt_password(&body.password).unwrap();
    let new_user = UserModel {
        _id: None,
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
        username: body.username.to_string(),
        email: body.email.to_string(),
        password: password_hash,
        created_at: Some(bson::DateTime::now()),
        updated_at: Some(bson::DateTime::now()),
    };
    let collection = client.database(&mongo_db).collection("user");
    let result = collection.insert_one(new_user, None).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
          "success": true
        })),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
