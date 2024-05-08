mod model;

use std::future::{ready, Ready};

use actix_web::{
    dev::Payload,
    error::InternalError,
    http::header,
    web::{get, post, Data, Json},
    App, FromRequest, HttpRequest, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use jwt_lib::UserDto;
use model::User;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use serde_json::json;
use std::env;

async fn create_username_index(client: &Client) {
    dotenv().ok();
    let mongo_db = env::var("MONGO_DB").expect("Can't get mongo db name");
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(&mongo_db)
        .collection::<User>("user")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    create_username_index(&client).await;

    let server_port = env::var("SERVER_PORT").expect("Can't get server port");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .route("/public-view", get().to(public_view_handler))
            .route("/get-token", post().to(get_token_handler))
            .route("/secret-view", get().to(secret_view_handler))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", server_port))
    .expect("Address should be free and valid")
    .run()
    .await
}

async fn public_view_handler() -> HttpResponse {
    HttpResponse::Ok().json(json!({
      "success": true,
      "data": {
        "message": "This data is visible to all users"
      }
    }))
}

async fn get_token_handler(client: Data<Client>, Json(user_dto): Json<UserDto>) -> HttpResponse {
    dotenv().ok();
    let mongo_db = env::var("MONGO_DB").expect("Can't get mongo db name");
    let collection: Collection<User> = client.database(&mongo_db).collection("user");
    let email = &user_dto.email;
    match collection.find_one(doc! { "email": email }, None).await {
        Ok(Some(user)) => {
            print!("{:?}", user);
            let token = jwt_lib::get_jwt(user_dto);
            return match token {
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
            };
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

async fn secret_view_handler(Auth(user): Auth) -> HttpResponse {
    HttpResponse::Ok().json(json!({
      "success": true,
      "email": user.email
    }))
}

struct Auth(UserDto);

impl FromRequest for Auth {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(" ").nth(1));

        match access_token {
            Some(token) => {
                let user = jwt_lib::decode_jwt(token);

                match user {
                    Ok(user) => ready(Ok(Auth(user))),

                    Err(e) => ready(Err(InternalError::from_response(
                        e.clone(),
                        HttpResponse::Unauthorized().json(json!({
                          "success": false,
                          "data": {
                            "message": e
                          }
                        })),
                    ))),
                }
            }

            None => ready(Err(InternalError::from_response(
                String::from("No token provided"),
                HttpResponse::Unauthorized().json(json!({
                  "success": false,
                  "data": {
                    "message": "No token provided"
                  }
                })),
            ))),
        }
    }
}
