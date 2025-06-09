use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use sqlx::{Pool, Postgres};

mod config;
mod models;
mod services;
mod routes;

#[derive(sqlx::FromRow)]
struct HealthCheck {
    #[allow(dead_code)]
    check: i32,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

async fn db_health_check(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "message": "Database connected"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Database connection failed: {}", e)
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    // Establish database connection
    let pool = config::database::establish_connection()
        .await
        .expect("Failed to connect to database");

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass database pool to the app
            .wrap(cors)
            .wrap(Logger::default())
            .service(web::scope("/api/auth")
                .route("/register", web::post().to(routes::auth::register_user_route))
                .route("/login", web::post().to(routes::auth::login_user_route))
            )
            .route("/health", web::get().to(health_check))
            .route("/db-health", web::get().to(db_health_check))
    })
    .bind((host, port))?
    .run()
    .await
} 