use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::GlobalConfig, routes::fetch_profile, utils::{rate_limit_middleware, RateLimiter}};

#[derive(Clone)]
pub struct AppContext {
    pub config: GlobalConfig,
    pub rate_limiter: RateLimiter,
}

#[derive(OpenApi)]
#[openapi(
    paths(crate::routes::fetch_profile,),
    components(schemas(crate::dtos::ApiResponse)),
    info(title = "HNG (Stage 0) Backend API", version = "1.0.0",)
)]
struct ApiDoc;

pub async fn build_app(context: AppContext) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            context
                .config
                .cors_allowed_origins
                .iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([axum::http::Method::GET])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::CACHE_CONTROL,
        ])
        .allow_credentials(true);

    Router::new()
        .route("/me", get(fetch_profile))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(axum::middleware::from_fn(rate_limit_middleware))
        .layer(axum::Extension(context.rate_limiter.clone()))
        .layer(cors)
        .with_state(context)
}
