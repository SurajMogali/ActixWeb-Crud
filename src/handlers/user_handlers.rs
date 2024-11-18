// use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
// use serde_json::json;
// use crate::{model::user::{User, UserLogin}, service::user_service::UserService};
// use crate::jwt::Claims;

// // Registration Handler
// #[post("/register")]
// pub async fn create_user(service: web::Data<UserService>, user: web::Json<User>) -> impl Responder {
//     match service.create_user(user.into_inner()).await {
//         Ok(new_user) => HttpResponse::Ok().json(new_user),
//         Err(err) => HttpResponse::BadRequest().json(json!({"error": err.to_string()})),
//     }
// }

// // Login Handler
// #[post("/login")]
// pub async fn login(service: web::Data<UserService>, credentials: web::Json<UserLogin>) -> impl Responder {
//     match service.login(&credentials.email, &credentials.password).await {
//         Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
//         Err(err) => HttpResponse::Unauthorized().json(json!({"error": err.to_string()})),
//     }
// }

// // Get Current User Handler
// #[get("/me")]
// pub async fn get_current_user(
//     service: web::Data<UserService>,
//     claims: Claims,
// ) -> impl Responder {
//     let user_id = claims.sub.clone();

//     match service.get_user_by_id(&user_id).await {
//         Ok(Some(user)) => HttpResponse::Ok().json(user),
//         Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
//         Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error fetching user"})),
//     }
// }

// // Update Current User Handler
// #[put("/update")]
// pub async fn update_current_user(
//     service: web::Data<UserService>,
//     claims: Claims,
//     user: web::Json<User>,
// ) -> impl Responder {
//     let user_id = claims.sub.clone();

//     match service.update_user(&user_id, user.into_inner()).await {
//         Ok(Some(updated_user)) => HttpResponse::Ok().json(json!({
//             "message": "User updated",
//             "user": updated_user
//         })),
//         Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
//         Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Error updating user"})),
//     }
// }

// // Delete Current User Handler
// #[delete("/delete")]
// pub async fn delete_current_user(
//     service: web::Data<UserService>,
//     claims: Claims,
// ) -> impl Responder {
//     let user_id = claims.sub.clone();

//     match service.delete_user(&user_id).await {
//         Ok(Some(_)) => HttpResponse::Ok().json(json!({"message":"User deleted"})),
//         Ok(None) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
//         Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error deleting user"})),
//     }
// }

// // Get All Users Handler (Protected, consider restricting to admins)
// #[get("/all")]
// pub async fn get_all_users(service: web::Data<UserService>) -> impl Responder {
//     match service.get_all_users().await {
//         Ok(users) => HttpResponse::Ok().json(users),
//         Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error fetching users"})),
//     }
// }
