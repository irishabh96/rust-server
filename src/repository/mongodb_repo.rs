use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::TryStreamExt;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId},
    results::{ InsertOneResult},
    Client, Collection,
};
use mongodb::bson::doc;
use mongodb::results::{DeleteResult, UpdateResult};
use crate::models::user_model::User;

pub struct MongoRepo{
    col: Collection<User>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

         let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable {}",""),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rust");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error>{
        let new_doc = User {
            id: None,
            name:   new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let user = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user details");
        Ok(user.unwrap())
    }

    pub async fn edit_user(&self, id: &String, updated_user: User) -> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let new_doc = doc! {
                "$set":
                    {
                        "id": updated_user.id,
                        "name": updated_user.name,
                        "location": updated_user.location,
                        "title": updated_user.title
                    },
            };

        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user details");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let deleted_doc = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user details");
        Ok(deleted_doc)
    }

    pub async fn get_all(&self) -> Result<Vec<User>, Error>{
        let mut cursor = self.col.find(None, None).await.ok().expect("Error finding users");

        let mut users: Vec<User> = Vec::new();

        while let Some(user) = cursor.try_next().await.ok().expect("Error mapping users"){
            users.push(user)
        }
        Ok(users)
    }
}