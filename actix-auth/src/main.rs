mod auth;
mod models;
mod routes;

use actix_web::{
    web::{get, Data},
    App, HttpResponse, HttpServer,
};

use dotenv::dotenv;
use models::user::UserModel;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use routes::config::config;
use serde_json::json;
use std::env;

async fn create_username_index(client: &Client) {
    let mongo_db = env::var("MONGO_DB").expect("Can't get mongo db name");
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(&mongo_db)
        .collection::<UserModel>("user")
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

    println!("Server started successfully 🚀!");
    println!("0.0.0.0:{}", server_port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .route("/secret-view", get().to(secret_view_handler))
            .configure(config)
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", server_port))
    .expect("Address should be free and valid")
    .run()
    .await
}

async fn secret_view_handler() -> HttpResponse {
    HttpResponse::Ok().json(json!({
      "success": true,
    }))
}
