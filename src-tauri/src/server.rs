use std::path::PathBuf;

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower_http::cors;

use crate::cache::build_cache;
use crate::fonts;

async fn get_font_handler(Path(postscript_name): Path<String>) -> Response {
    match fonts::get_font(&postscript_name) {
        Ok((font_data, content_type)) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            font_data,
        )
            .into_response(),
        _ => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn post_refresh_handler(static_dir: PathBuf) -> Response {
    match build_cache(&static_dir) {
        Ok(_) => {
            tracing::info!("Cache refreshed successfully");
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            tracing::error!("Error building cache: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn start(static_dir: &PathBuf) {
    let static_dir_clone = static_dir.clone();

    let router = axum::Router::new()
        .route("/", get(|| async { "OK" }))
        .nest_service("/v1", tower_http::services::ServeDir::new(static_dir))
        .route(
            "/v1/refresh",
            post(|| async move { post_refresh_handler(static_dir_clone).await }),
        )
        .route("/v1/fonts/{postscript_name}", get(get_font_handler))
        .layer(cors::CorsLayer::new().allow_origin(cors::Any))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    tracing::info!("Starting server on 0.0.0.0:36687");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:36687")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap()
}
