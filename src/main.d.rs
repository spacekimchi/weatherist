#[macro_use]
extern crate actix_web;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use std::{env, io};
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use actix_web::{middleware, App, web, HttpServer};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    dotenv().ok();
    println!("Connecting to database: {}", std::env::var("DATABASE_URL").unwrap());
    let database_url = std::env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a conneciton pool");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
