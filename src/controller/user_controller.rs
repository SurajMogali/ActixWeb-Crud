use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{model::user::User, service::user_service::UserService};

#[post("/users")]
pub async fn create_user(service: web::Data<UserService>, user: web::Json<User>) -> impl Responder {
    match service.create_user(user.into_inner()).await {
        Ok(new_user) => HttpResponse::Ok().json(new_user),
        Err(err) => HttpResponse::BadRequest().json(json!({"error":err.to_string()})),
    }
}



#[get("/getUsers")]
pub async fn get_user(
    service: web::Data<UserService>,
    req:HttpRequest
) -> impl Responder {

let user_id=match req.headers().get("user_id"){
    Some(id)=>id.to_str().unwrap_or(""),
    None=>"",
};
if user_id.is_empty(){
    return HttpResponse::BadRequest().json(json!({"error":"user_id missing in headers"}));

}


    match service.get_user_by_id(&user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error fetching user"})),
    }
}


#[put("/updateUser")]
pub async fn update_user(
    service: web::Data<UserService>,
    req:HttpRequest,
    user: web::Json<User>,
) -> impl Responder {


    let user_id =match req.headers().get("user_id"){
        Some(id)=>id.to_str().unwrap_or(""),
        None=>"",
    };

    if user_id.is_empty(){
        return HttpResponse::BadRequest().json(json!({"error":"user_id missing in headers"}));
    }



    match service.update_user(&user_id, user.into_inner()).await {
        Ok(Some(updated_user)) => HttpResponse::Ok().json(json!({
            "message":"User updated",
            "user":updated_user
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error updating user"})),
    }
}

#[delete("/deleteUser")]
pub async fn delete_user(
    service: web::Data<UserService>,
    req:HttpRequest,
) -> impl Responder {

    let user_id = match req.headers().get("user_id") {
        Some(id) => id.to_str().unwrap_or(""),
        None => "",
    };

    if user_id.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "user_id missing in headers"}));
    }

    match service.delete_user(&user_id).await {
        Ok(Some(_)) => HttpResponse::Ok().json(json!({"message":"User deleted"})),
        Ok(None) => HttpResponse::NotFound().json(json!({"error":"User not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error deleting user"})),
    }
}


#[get("/getAllUsers")]
pub async fn get_all_users(service: web::Data<UserService>) -> impl Responder {
    match service.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error":"Error fetching users"})),
    }
}
