use mongodb::{Collection, bson::{doc, oid::ObjectId}};
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;
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
}

impl User {
    pub fn new(
        id: ObjectId,
        name: String,
        email: String,
        password: String,
        created_at: String,
        updated_at: String,
    ) -> User {
        User {
            id,
            name,
            email,
            password,
            created_at,
            updated_at,
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
        );
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

    pub async fn get_all_active_users(user_collection: Collection<User>) -> Vec<User> {
        let filter = doc! { "active": true };
        let users = match user_collection.find(filter, None).await {
            Ok(users) => {
                // cursor.unwrap().try_collect::<Vec<_>>().await
                match users.try_collect::<Vec<_>>().await {
                    Ok(users) => {
                        println!("Users found {:?}", users);
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

}