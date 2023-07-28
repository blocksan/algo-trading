use mongodb::Collection;
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn new(
        id: i32,
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

    pub fn get_by_id(&self) -> i32 {
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
}