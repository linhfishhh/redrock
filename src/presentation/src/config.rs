use std::rc::Rc;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};
use deadpool_postgres::Pool;
use tower_http::cors::{Any, CorsLayer};
use crate::api;
use crate::api::get_all;

pub fn configure(pool: &Pool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .nest("/pokemon", Router::new()
            .route("/", post())
            .route("/:id", get(get_all))
        )
        .layer(cors)
        .with_state(pool)
}