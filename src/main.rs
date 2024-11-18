use crate::controller::user_controller;
use actix_web::{web, App, HttpServer};


use controller::user_controller::{create_user, delete_user, get_all_users, get_user, update_user};
use dotenv::dotenv;
use service::user_service::UserService;
use std::env;


mod controller;
mod service;
mod routes;
mod model;
mod handlers;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let user_service = UserService::new(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            
            
            
        
           
           
    })
    .bind("localhost:8089")?
    .run()
    .await
}
