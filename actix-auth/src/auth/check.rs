use actix_web::{
    dev::Payload, error::InternalError, http::header, FromRequest, HttpRequest, HttpResponse,
};
use jwt_lib::UserDto;
use serde_json::json;
use std::future::{ready, Ready};

pub struct Auth(pub UserDto);

impl FromRequest for Auth {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split(' ').nth(1));

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
