use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};

use crate::err::AppErr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub name: String,
    pub exp: usize,
}

impl Claims {
    pub fn default() -> Self {
        Self {
            id: "".to_owned(),
            name: "".to_owned(),
            exp: 0,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_owned();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn exp_from_now(mut self, dur: Duration) -> Self {
        self.exp = (Utc::now() + dur).timestamp() as usize;
        self
    }
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("SERVER_SECRET_KEY").expect("SERVER_SECRET_KEY NOT SET"),
        }
    }

    pub fn issue(&self, claims: &Claims) -> Result<String, AppErr> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AppErr::JwtEncodingError)
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AppErr> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|err| {
            match err.kind() {
                ErrorKind::ExpiredSignature => AppErr::ExpiredToken,
                _ => AppErr::InvalidToken
            }
        })?;

        Ok(token_data.claims)
    }
}
