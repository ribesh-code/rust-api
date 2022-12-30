// add the modules
mod api;
mod models;
mod repository;
use std::env;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
