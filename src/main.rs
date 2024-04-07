mod api;
mod models;
mod repository;

use actix_web::{web::Data, HttpServer, App};
use api::user_api::{create_user};
use repository::mongodb_repo::MongoRepo;
use crate::api::user_api::{delete_user, get_all, get_user, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
        HttpServer::new(move || {
            App::new()
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