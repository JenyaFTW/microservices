use std::env;
use std::error::Error;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Duration, Utc};
use chrono::format::Numeric::Day;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    id: Uuid,
    email: String,
    iat: i64,
    exp: i64
}

impl User {
    pub async fn new(name: String, email: String, password: String) -> Result<Self, String> {
        Ok(User {
            id: Uuid::new_v4(),
            updated_at: Utc::now(),
            created_at: Utc::now(),
            name,
            email,
            password
        })
    }

    pub async fn to_jwt(self) -> String {
        let claims = UserClaims {
            id: self.id,
            email: self.email,
            iat: Utc::now().timestamp_millis(),
            exp: Utc::now().add(Duration::days(7)).timestamp_millis()
        };

        let secret = env::var("JWT_SECRET").unwrap_or("totallysecret".to_string());
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
        return token;
    }
}
