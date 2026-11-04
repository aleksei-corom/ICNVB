use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod store;

use handlers::{
    add_monthly_event, create_prayer_request, get_church_info, get_prayer_requests, get_schedule,
    health_check,
};
use store::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "icnvb_backend=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| "../data".to_string());
    let state = AppState::new(&data_dir);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(get_church_info))
        .route("/schedule", get(get_schedule).post(add_monthly_event))
        .route(
            "/prayer-request",
            post(create_prayer_request).get(get_prayer_requests),
        )
        .layer(cors);

    // Servir archivos estáticos del frontend si existe ../frontend/dist
    let dist_dir = PathBuf::from("../frontend/dist");
    let app = if dist_dir.exists() {
        tracing::info!("Servidor sirviendo archivos estáticos desde {:?}", dist_dir);
        Router::new()
            .nest("/api", api_routes)
            .fallback_service(ServeDir::new(dist_dir))
            .with_state(state)
    } else {
        tracing::info!("Modo API: 'frontend/dist' no encontrado todavía, sirviendo solo API.");
        Router::new()
            .nest("/api", api_routes)
            .route("/", get(|| async { "ICNVB API está en línea. Frontend en desarrollo o ejecutable en puerto independiente." }))
            .with_state(state)
    };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🕊️ Servidor Iglesia Cristiana Nueva Vida corriendo en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
