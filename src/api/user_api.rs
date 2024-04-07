use crate::models::user_model::User;
use crate::repository::mongodb_repo::MongoRepo;

use actix_web::{post, get, web::{Data, Json, Path}, HttpResponse, put, delete};
use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    println!("Creating user with : {:#?}", data);

    let user_details = db.create_user(data).await;
    
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {

    let id = path.into_inner();
    println!("fetching user details for id {id}");

    if id.is_empty() || id.len() < 24 {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let user_detail = db.get_user(&id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(db: Data<MongoRepo>, path: Path<String>, new_user: Json<User>) -> HttpResponse {

    let id = path.into_inner();
    println!("updating user details for id {id}");

    if id.is_empty() || id.len() < 24 {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let update_result = db.edit_user(&id, data).await;
    match update_result {
        Ok(update) => {
            return if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                 match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    println!("deleting user details for id {id}");

    if id.is_empty() || id.len() < 24 {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let deleted_user = db.delete_user(&id).await;

    match deleted_user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all(db: Data<MongoRepo>) -> HttpResponse {
    let all_users = db.get_all().await;

    match all_users {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}