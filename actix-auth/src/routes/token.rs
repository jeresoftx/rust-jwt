use crate::models::user::UserModel;

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use jwt_lib::UserDto;
use serde_json::json;

use mongodb::{bson::doc, Client, Collection};

use std::env;

#[post("/token")]
async fn get_token_handler(client: Data<Client>, Json(user_dto): Json<UserDto>) -> HttpResponse {
    let mongo_db = env::var("MONGO_DB").expect("Can't get mongo db name");
    let collection: Collection<UserModel> = client.database(&mongo_db).collection("user");
    let email = &user_dto.email;
    match collection.find_one(doc! { "email": email }, None).await {
        Ok(Some(user)) => {
            let token = jwt_lib::get_jwt(user_dto);
            match token {
                Ok(token) => HttpResponse::Ok().json(json!({
                  "success": true,
                  "data": {
                    "email": user.email,
                    "token": token
                  }
                })),

                Err(e) => HttpResponse::BadRequest().json(json!({
                  "success": false,
                  "data": {
                    "message":  e.to_string()
                  }
                })),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
          "success": false,
          "data": {
            "message":  format!("No user found with email {email}")
          }
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
          "success": false,
          "data": {
            "message":  err.to_string()
          }
        })),
    }
}
