use serde::{Deserialize, Serialize};
use actix_web::web::Data;
use actix_web::HttpResponse;
use sqlx::{self, query_file};
use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct SeedRequest {
}

#[post("/seeds")]
pub async fn seed(state: Data<AppState>) -> HttpResponse {
    match query_file!("db/seeds/seeds.sql")
        .execute(&state.db)
        .await
        {
            Ok(res) => HttpResponse::Ok().body(format!("Successfully seeded the database: {:#?}", res)),
            Err(err) => HttpResponse::InternalServerError().json(format!("Failed to seed database: {err}")),
        }
}

#[post("/spots_seed")]
pub async fn spots_seed(state: Data<AppState>) -> HttpResponse {
    match query_file!("db/seeds/spots_seed.sql")
        .execute(&state.db)
        .await
        {
            Ok(res) => HttpResponse::Ok().body(format!("Successfully seeded spots the database: {:#?}", res)),
            Err(err) => HttpResponse::InternalServerError().json(format!("Failed to seed database: {err}")),
        }
}

#[post("/migrate")]
pub async fn migrate(state: Data<AppState>) -> HttpResponse {
    match sqlx::migrate!("db/migrations")
        .run(&state.db)
        .await
        {
            Ok(res) => HttpResponse::Ok().body(format!("Successfully migrated the database: {:#?}", res)),
            Err(err) => HttpResponse::InternalServerError().json(format!("Failed to migrate database: {err}")),
        }
}
