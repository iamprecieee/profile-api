use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub email: String,
    pub name: String,
    pub stack: String,
}

impl UserProfile {
    pub fn new(email: String, full_name: String, stack: String) -> Self {
        Self {
            email: email,
            name: full_name,
            stack: stack,
        }
    }
}

#[derive(Deserialize)]
pub struct CatFactResponse {
    pub fact: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse {
    pub status: String,
    pub user: UserProfile,
    pub timestamp: String,
    pub fact: String,
}

impl ApiResponse {
    pub fn new(profile_data: UserProfile, fact: String) -> Self {
        Self {
            status: String::from("success"),
            user: profile_data,
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            fact: fact,
        }
    }
}
