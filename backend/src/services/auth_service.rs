use crate::models::user::{CreateUser, User, LoginUser};
use sqlx::{Pool, Postgres};
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;
use uuid::Uuid;

// JWT Claims struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // Subject (user ID)
    pub exp: usize, // Expiration time
}

// Function to hash a password
pub async fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

// Function to verify a password
pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    Ok(verify(password, hashed_password)?)
}

// Function to generate a JWT token
pub fn generate_token(user_id: Uuid) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration_time = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration_time,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ).map_err(|e| anyhow!("Failed to generate token: {}", e))
}

#[allow(dead_code)]
pub fn validate_token(token: &str) -> Result<Claims> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map(|data| data.claims)
    .map_err(|e| anyhow!("Invalid token: {}", e))
}

pub async fn register_user(pool: &Pool<Postgres>, new_user: CreateUser) -> Result<User> {
    let hashed_password = hash_password(&new_user.password_hash).await?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        new_user.username,
        new_user.email,
        hashed_password
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn login_user(pool: &Pool<Postgres>, login_user: LoginUser) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", login_user.email)
        .fetch_optional(pool)
        .await?;

    match user {
        Some(user) => {
            if verify_password(&login_user.password_hash, &user.password_hash).await? {
                Ok(user)
            } else {
                Err(anyhow!("Invalid credentials"))
            }
        }
        None => Err(anyhow!("Invalid credentials")),
    }
}