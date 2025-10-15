use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    api::AppContext,
    dtos::{ApiResponse, UserProfile},
    utils::get_random_cat_fact,
};

#[utoipa::path(
    get,
    path = "/me",             
    tag = "Profile",               
    responses((
        status = 200,
        description = "Fetch user profile with random cat fact",
        body = ApiResponse
    ))
)]
pub async fn fetch_profile(State(context): State<AppContext>) -> impl IntoResponse {
    let cat_fact = get_random_cat_fact(context.config.cat_fact_api).await;

    let profile_data = UserProfile {
        email: context.config.email,
        name: context.config.full_name,
        stack: context.config.stack,
    };

    let response_data = ApiResponse::new(profile_data, cat_fact);

    (StatusCode::OK, Json(response_data)).into_response()
}
