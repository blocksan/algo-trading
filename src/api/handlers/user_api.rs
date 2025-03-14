use actix_web::{HttpResponse, post, web, Responder, get};
use serde::Deserialize;
use crate::user::user::User;

#[derive(Deserialize, Debug)]
struct CreateUserBodyParams {
    name: String,
    email: String,
    password: String,
}

/// Creates a new user in the system
///
/// @path /create_user
/// @method POST
/// @tag Users
/// @request_body {
///   "name": "John Doe",
///   "email": "john@example.com",
///   "password": "securepassword123"
/// }
/// @response 200 {
///   "message": "New user created"
/// }
#[post("/create_user")]
async fn create_user(body: web::Json<CreateUserBodyParams>) -> impl Responder {
    User::add_new_user_via_db(body.name.clone(), body.email.clone(), body.password.clone()).await;
    HttpResponse::Ok().body("New user created")
}

/// Retrieves a user by their email address
///
/// @path /fetch_user/{email}
/// @method GET
/// @tag Users
/// @param email path string true "Email address of the user"
/// @response 200 {
///   "id": "64d8febebe3ea57f392c36df",
///   "name": "John Doe",
///   "email": "john@example.com",
///   "created_at": "2023-08-13 14:30:45",
///   "updated_at": "2023-08-13 14:30:45",
///   "active": true
/// }
#[get("/fetch_user/{email}")]
async fn fetch_user(path: web::Path<String>) -> impl Responder {
    print!("email: {}", path);
    let email = path.into_inner();
    let user = User::get_user_by_email_via_db(email).await;
    if user.is_some(){
        let user = user.unwrap();
       HttpResponse::Ok().json(user)
   }else{
       HttpResponse::Ok().body("No user found")
   }
}

//TODO: Add update user
// #[post("/update_user")]
// async fn update_user(body: web::Json<CreateUserBodyParams>) -> impl Responder {
//     User::add_new_user_via_db(body.name.clone(), body.email.clone(), body.password.clone()).await;
//     HttpResponse::Ok().body("New user created")
// }