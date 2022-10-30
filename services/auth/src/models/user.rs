use crate::lib::rest_response::RestResponse;
use crate::routes::AppState;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::TypedHeader;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Pool, Postgres};
use std::env;
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: Uuid,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
    AlreadyExists,
    DbError,
}

impl User {
    pub async fn new(
        db: &Pool<Postgres>,
        email: String,
        password: String,
    ) -> Result<Self, UserError> {
        let hashed_password = hash(&password, DEFAULT_COST).unwrap();
        let existing_user = User::from_email(db, email.clone()).await;

        match existing_user {
            Ok(_) => return Err(UserError::AlreadyExists),
            Err(_) => (),
        };

        let user_result = query_as!(
            User,
            r"
            INSERT INTO users (email, password)
            VALUES ($1, $2)
            RETURNING *
            ",
            email,
            hashed_password
        )
        .fetch_one(db)
        .await;

        match user_result {
            Ok(u) => Ok(u),
            Err(_) => Err(UserError::DbError),
        }
    }

    pub async fn from_id(db: &Pool<Postgres>, id: Uuid) -> Result<Self, UserError> {
        let user_result = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(db)
            .await;

        match user_result {
            Ok(r) => Ok(r),
            Err(_) => Err(UserError::NotFound),
        }
    }

    pub async fn from_email(db: &Pool<Postgres>, email: String) -> Result<Self, UserError> {
        let user_result = query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(db)
            .await;

        match user_result {
            Ok(r) => Ok(r),
            Err(_) => Err(UserError::NotFound),
        }
    }

    pub async fn to_jwt(self) -> String {
        let key =
            EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET missing").as_ref());
        let claims = UserClaims {
            sub: self.id,
            email: self.email,
            iat: Utc::now().timestamp_millis(),
            exp: Utc::now().add(Duration::days(7)).timestamp_millis(),
        };

        encode(&Header::default(), &claims, &key).unwrap()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = RestResponse<()>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header =
            match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
                Ok(h) => h,
                Err(_) => {
                    return Err(RestResponse::with_message(
                        StatusCode::FORBIDDEN,
                        "Missing Authorization header".to_string(),
                    ))
                }
            };

        let key =
            DecodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET missing").as_ref());

        return match decode::<UserClaims>(
            &auth_header.token(),
            &key,
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(c) => Ok(c.claims),
            Err(_) => Err(RestResponse::with_message(
                StatusCode::FORBIDDEN,
                "Error validating JWT token".to_string(),
            )),
        };
    }
}
