/*
use serde::{Deserialize, Serialize};
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use uuid::Uuid;
//use actix_web::web::Path;

use sqlx::{self, FromRow};
use crate::response::Response;
use crate::AppState;
use crate::ingredient::Ingredient;

pub type Recipes = Response<Recipe>;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub instructions: Vec<String>,
    pub body: String,
    pub ingredients: Vec<String>,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeRequest {
    pub instructions: Vec<String>,
    pub body: String,
    pub ingredients: Vec<String>,
    pub url: String,
}

impl Recipe { pub fn new(instructions: Vec<String>, body: String, ingredients: Vec<String>, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: chrono::offset::Utc::now(),
            instructions,
            body,
            ingredients,
            url,
        }
    }
}

#[get("/recipes")]
pub async fn list(state: Data<AppState>) -> impl Responder {
    // TODO: get recipes this will have query params "?ingredients=apple,chicken,thyme"
    match sqlx::query_as::<_, Recipe>("SELECT id, ingredients, url, instructions, body, created_at FROM recipes")
        .fetch_all(&state.db)
        .await
        {
            Ok(recipes) => HttpResponse::Ok().content_type("application/json").json(recipes),
            Err(_) => HttpResponse::NotFound().json("No recipes found"),
        }
}

#[post("/recipes")]
pub async fn create(state: Data<AppState>, body: Json<RecipeRequest>) -> HttpResponse {
    let id = uuid::Uuid::new_v4();
    let created_at = chrono::offset::Utc::now();
    println!("id: {}, ingredients: {:#?}, url: {}, instructions: {:#?}, body: {}, created_at: {}", id, body.ingredients, body.url, body.instructions, body.body, created_at);
    match sqlx::query_as::<_, Recipe>(
        "INSERT INTO recipes (id, ingredients, url, instructions, body, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, ingredients, url, instructions, body, created_at"
    )
    .bind(id)
    .bind(body.ingredients.clone())
    .bind(body.url.to_string())
    .bind(body.instructions.clone())
    .bind(body.body.clone())
    .bind(created_at)
    .fetch_one(&state.db)
    .await
    {
        Ok(recipe) => HttpResponse::Ok().json(recipe),
        Err(err) => HttpResponse::InternalServerError().json(format!("Failed to create recipe: {err}")),
    }
}

#[get("/recipes/{recipe_id}")]
pub async fn get(state: Data<AppState>, path: Path<String>) -> impl Responder {
    // TODO: Get recipe by ID. This will discard query params
    let recipe_id = path.into_inner();
    match sqlx::query_as::<_, Recipe>("SELECT id, ingredients, url, instructions, body, created_at FROM recipes WHERE id = $1")
        .bind(recipe_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(recipe) => HttpResponse::Ok().json(recipe),
            Err(_) => HttpResponse::NotFound().json("No recipes found"),
        }
}

#[delete("/recipes/{recipe_id}")]
pub async fn delete(state: Data<AppState>, path: Path<(String,)>) -> HttpResponse {
    // TODO: Delete recipe by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}
*/
