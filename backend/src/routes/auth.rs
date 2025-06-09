use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use crate::models::user::{CreateUser, LoginUser};
use crate::services::auth_service;

pub async fn register_user_route(pool: web::Data<Pool<Postgres>>, new_user: web::Json<CreateUser>) -> impl Responder {
    match auth_service::register_user(pool.get_ref(), new_user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "User registered successfully",
            "user_id": user.id,
            "username": user.username,
            "email": user.email,
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Failed to register user: {}", e.to_string())
        })),
    }
}

pub async fn login_user_route(pool: web::Data<Pool<Postgres>>, login_user: web::Json<LoginUser>) -> impl Responder {
    match auth_service::login_user(pool.get_ref(), login_user.into_inner()).await {
        Ok(user) => {
            match auth_service::generate_token(user.id) {
                Ok(token) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "message": "Logged in successfully",
                    "token": token,
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to generate token: {}", e.to_string())
                })),
            }
        }
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "status": "error",
            "message": format!("Authentication failed: {}", e.to_string())
        })),
    }
} 