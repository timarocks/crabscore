//! Lightweight Axum dashboard for CrabScore reports.

use axum::{extract::State, http::StatusCode, response::Redirect, routing::get, Router};
use serde_json::Value;
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;

use crate::generator::generate_json;
use crabscore_core::CrabScore;

#[derive(Clone)]
struct AppState {
    score: Arc<CrabScore>,
}

/// Start a blocking web server on the given address.
pub async fn serve(score: CrabScore, addr: SocketAddr) -> anyhow::Result<()> {
    let state = AppState {
        score: Arc::new(score),
    };

    // Routes
    let app = Router::new()
        .route("/", get(root))
        .route("/data.json", get(data))
        .with_state(state)
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        );

    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> Redirect {
    axum::response::Redirect::permanent("/static/report.html")
}

async fn data(State(state): State<AppState>) -> Result<axum::Json<Value>, StatusCode> {
    let json = generate_json(&state.score);
    Ok(axum::Json(serde_json::to_value(json).unwrap()))
}
