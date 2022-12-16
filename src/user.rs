use serde::{Deserialize, Serialize};
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use uuid::Uuid;
//use actix_web::web::Path;

use sqlx::{self, FromRow};
use crate::response::Response;
use crate::AppState;

pub type Users = Response<User>;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: 0,
            created_at: chrono::offset::Utc::now(),
            username,
            email,
        }
    }
}

#[get("/users")]
pub async fn list(state: Data<AppState>) -> impl Responder {
    // TODO: get users this will have query params "?ingredients=apple,chicken,thyme"
    println!("[users.rs:list]");
    match sqlx::query_as::<_, User>("SELECT id, username, email, created_at FROM users")
        .fetch_all(&state.db)
        .await
        {
            Ok(users) => HttpResponse::Ok().content_type("application/json").json(users),
            Err(_) => HttpResponse::NotFound().json("No users found"),
        }
}

#[post("/users")]
pub async fn create(state: Data<AppState>, body: Json<UserRequest>) -> HttpResponse {
    let id = uuid::Uuid::new_v4();
    let created_at = chrono::offset::Utc::now();
    println!("id: {}, username: {}, email: {}, created_at: {}", id, body.username, body.email, created_at);
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, created_at) VALUES ($1, $2, $3, $4) RETURNING id, username, email, created_at"
    )
    //.bind(id)
    .bind(body.username.clone())
    .bind(body.email.to_string())
    .bind(created_at)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed to create user: {err}")),
    }
}

#[get("/users/{user_id}")]
pub async fn get(state: Data<AppState>, path: Path<String>) -> impl Responder {
    // TODO: Get user by ID. This will discard query params
    let user_id = path.into_inner();
    match sqlx::query_as::<_, User>("SELECT id, username, email, created_at FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().json("No user found"),
        }
}

#[delete("/users/{user_id}")]
pub async fn delete(state: Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    // TODO: Delete user by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}
