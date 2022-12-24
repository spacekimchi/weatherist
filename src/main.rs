#[macro_use]
extern crate actix_web;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use std::{env, io};
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::{middleware, App, web, HttpServer};

//mod recipe;
//mod response;
mod user;
mod spot;
mod response;
mod seeds;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    dotenv().ok();
    println!("Connecting to database: {}", std::env::var("DATABASE_URL").unwrap());
    /* DATABASE://DATABASE_USER:DATABASE_PASSWORD@DATABASE_HOST:DATABASE_PORT/DATABASE_DB_NAME */
    let _database = std::env::var("DATABASE").expect("DATABSE must be set");
    let _database_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let _database_password = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let _database_host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let _database_port = std::env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
    let _database_name = std::env::var("DATABASE_NAME").expect("DATABSE_NAME must be set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    println!("database_url: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a conneciton pool");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(user::create)
            .service(user::get)
            .service(user::list)
            .service(user::delete)
            .service(spot::create)
            .service(spot::get)
            .service(spot::list)
            .service(spot::delete)
            .service(seeds::seed)
            .service(seeds::spots_seed)
            .service(seeds::migrate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
