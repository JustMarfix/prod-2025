use crate::errors::ProdError;
use crate::jwt::generate::validate_token;
use crate::models::RoleModel;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

const JWT_EXPIRY_HOURS: i64 = 1;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub role: RoleModel,
    pub id: Uuid,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: &Uuid, role: &RoleModel) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(JWT_EXPIRY_HOURS);

        Self {
            role: role.clone(),
            id: *user_id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

impl FromStr for Claims {
    type Err = ProdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let claims: Vec<_> = s.split(' ').collect();
        let token = claims.get(1).ok_or(ProdError::ShitHappened(
            "Wrong authorization Bearer format".to_string(),
        ))?;
        validate_token(token)
    }
}