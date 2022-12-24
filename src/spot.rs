use serde::{Deserialize, Serialize};
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
//use actix_web::web::Path;

use sqlx::{self, FromRow};
//use crate::response::Response;
use crate::AppState;

//pub type Spots = Response<Spot>;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Spot {
    pub id: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub name: String,
    pub shared: bool,
    pub content: String,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotRequest {
    pub name: String,
    pub shared: bool,
    pub content: String,
    pub user_id: i32,
}

#[get("/spots")]
pub async fn list(state: Data<AppState>) -> impl Responder {
    // TODO: get spots this will have query params "?ingredients=apple,chicken,thyme"
    match sqlx::query_as::<_, Spot>("SELECT id, name, shared, content, user_id, created_at FROM spots")
        .fetch_all(&state.db)
        .await
        {
            Ok(spots) => HttpResponse::Ok().content_type("application/json").json(spots),
            Err(err) => HttpResponse::NotFound().json(format!("Error: {err}")),
        }
}

#[post("/spots")]
pub async fn create(state: Data<AppState>, body: Json<SpotRequest>) -> HttpResponse {
    let created_at = chrono::offset::Utc::now();
    match sqlx::query_as::<_, Spot>(
        "INSERT INTO spots (name, shared, content, user_id, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, name, shared, content, user_id, created_at"
    )
    .bind(body.name.clone())
    .bind(body.shared)
    .bind(body.content.to_string())
    .bind(body.user_id)
    .bind(created_at)
    .fetch_one(&state.db)
    .await
    {
        Ok(spot) => HttpResponse::Ok().json(spot),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed to create spot: {err}")),
    }
}

#[get("/spots/{spot_id}")]
pub async fn get(state: Data<AppState>, path: Path<String>) -> impl Responder {
    // TODO: Get spot by ID. This will discard query params
    let spot_id = path.into_inner();
    match sqlx::query_as::<_, Spot>("SELECT id, name, shared, content, user_id, created_at FROM spots WHERE id = $1")
        .bind(spot_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(spot) => HttpResponse::Ok().json(spot),
            Err(_) => HttpResponse::NotFound().json("No spot found"),
        }
}


#[delete("/spots/{spot_id}")]
pub async fn delete(_state: Data<AppState>, _path: Path<(String,)>) -> HttpResponse {
    // TODO: Delete spot by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}
