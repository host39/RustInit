//! Route modules. Each exposes `router()` with its OpenAPI-documented handlers; mount it here.

pub mod docs;
pub mod health;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::config::AppState;
use crate::routes::docs::ApiDoc;

/// Every documented route, collected into one router plus the OpenAPI document.
pub fn router() -> OpenApiRouter<AppState> {
    let mut document = ApiDoc::openapi();
    // utoipa fills the license from Cargo.toml; the template sets none, so publish none.
    document.info.license = None;
    OpenApiRouter::with_openapi(document).merge(health::router())
}
