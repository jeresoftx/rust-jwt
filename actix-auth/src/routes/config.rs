use actix_web::web;

use super::health_route::health_checker_handler;
use super::token::get_token_handler;
use super::user::add_user;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(add_user)
        .service(health_checker_handler)
        .service(get_token_handler);

    conf.service(scope);
}
