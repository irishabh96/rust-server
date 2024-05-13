mod api;
mod models;
mod repository;
use std::collections::HashSet;

use actix_web::{dev::ServiceRequest, error, middleware::Logger, web::Data, App, Error, HttpServer};
use actix_web_grants::GrantsMiddleware;
use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;
use serde::Serialize;
use crate::api::user_api::{delete_user, get_all, get_user, update_user};


const AUTHORIZATION_HEADER: &str = "Authorization";

#[derive(Serialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
}


pub async fn extract(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION_HEADER) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match token.len() == 16 {
                    true => {
                        println!("Valid token: {}", token);
                        Ok(HashSet::<String>::new())
                    }
                    false => {
                        eprintln!("Failed to verify JWT token: {}", token);
                        Err(error::ErrorBadRequest("Invalid token"))
                    }
                }
            } else {
                Err(error::ErrorBadRequest("Invalid token format"))
            }
        } else {
            Err(error::ErrorBadRequest("Invalid header value"))
        }
    } else {
        Err(error::ErrorBadRequest( "Missing authorization token"))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
         HttpServer::new(move || {
             App::new()
                .wrap(Logger::default())
                .wrap(GrantsMiddleware::with_extractor(
                    extract,
                ))
                // .wrap(GrantsMiddleware::with_extractor(extract))
                .app_data(db_data.clone())
                .service(create_user)
                .service(get_user)
                .service(update_user)
                .service(delete_user)
                .service(get_all)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}