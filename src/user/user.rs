use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
use crate::{config::mongodb_connection, common::date_parser};
// fetch_db_connection
#[allow(dead_code)]
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
    pub active: bool,
}

impl User {
    pub fn new(
        id: ObjectId,
        name: String,
        email: String,
        password: String,
        created_at: String,
        updated_at: String,
        active: bool,
    ) -> User {
        User {
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
            active
        }
    }

    pub fn get_by_id(&self) -> ObjectId {
        self.id
    }

    pub async fn add_new_user(&self, user_collection: Collection<User>){
        let user = User::new(
            self.id,
            self.name.clone(),
            self.email.clone(),
            self.password.clone(),
            self.created_at.clone(),
            self.updated_at.clone(),
            true,
        );

        let existing_user = User::get_user_by_email(&user.email, user_collection.clone()).await;

        if existing_user.is_some() {
            println!("User already exists");
            return;
        }

        match user_collection.insert_one(user.clone(), None).await {
            Ok(_) => {
                println!("Successfully inserted new User into the collection");
            },
            Err(e) => {
                println!("Error while inserting new User into the collection due to error {:?}",e);
            }
        }
        
    }
    pub async fn get_user_by_email(email: &str, user_collection: Collection<User>) -> Option<User> {
        let filter = doc! { "email": email.clone() };
        match user_collection.find_one(filter, None).await {
            Ok(user) => {
                match user {
                    Some(user) => {
                        println!("User found {:?}", user);
                        Some(user)
                    },
                    None => {
                        println!("User not found");
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error while finding user due to error {:?}", e);
                None
            }
        }
    }

    pub async fn get_all_active_users() -> Vec<User> {
        let user_collection = User::get_user_collection().await;
        let filter = doc! { "active": true };
        let users = match user_collection.find(filter, None).await {
            Ok(users) => {
                // cursor.unwrap().try_collect::<Vec<_>>().await
                match users.try_collect::<Vec<_>>().await {
                    Ok(users) => {
                        // println!("Users found {:?}", users);
                        Some(users)
                    },
                    Err(e) => {
                        println!("Error while finding users due to error {:?}", e);
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error while finding users due to error {:?}", e);
                None
            }
        };

        if users.is_none(){
            vec![]
        }else{
            users.unwrap()
        }
    }



    pub async fn add_new_user_via_db(name: String, email: String, password: String){
        let new_user = User::new(
            ObjectId::new(),
            name,
            email,
            password,
            date_parser::new_current_date_time_in_desired_stock_datetime_format(),
        date_parser::new_current_date_time_in_desired_stock_datetime_format(),
            true,
        );

        let user_collection = User::get_user_collection().await;

        let existing_user = User::get_user_by_email(&new_user.email, user_collection.clone()).await;

        if existing_user.is_some() {
            println!("User already exists");
            return;
        }

        match user_collection.insert_one(new_user.clone(), None).await {
            Ok(_) => {
                println!("Successfully inserted new User into the collection");
            },
            Err(e) => {
                println!("Error while inserting new User into the collection due to error {:?}",e);
            }
        }
        
    }

    pub async fn get_user_by_email_via_db(email: String) -> Option<User> {
        let user_collection = User::get_user_collection().await;

        let filter = doc! { "email": email.clone() };
        match user_collection.find_one(filter, None).await {
            Ok(user) => {
                match user {
                    Some(user) => {
                        println!("User found {:?}", user);
                        Some(user)
                    },
                    None => {
                        println!("User not found");
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error while finding user due to error {:?}", e);
                None
            }
        }
    }

    
    pub async fn get_user_collection() -> Collection<User>{
        let db = mongodb_connection::fetch_db_connection().await;
        let user_collection_name = "users";
        let user_collection = db.collection::<User>(user_collection_name);
        return user_collection
    }

}