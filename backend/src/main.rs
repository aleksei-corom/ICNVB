use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use http::header::HeaderValue;

mod handlers;
mod models;
mod store;

use handlers::{
    admin_change_password, admin_create_event, admin_delete_event, admin_delete_gallery_item,
    admin_export_events_csv, admin_export_events_json, admin_export_gallery_json,
    admin_export_prayers_json, admin_get_events, admin_get_gallery, admin_login,
    admin_update_event, admin_upload_gallery, auth_middleware, create_prayer_request,
    get_church_info, get_prayer_requests, get_schedule, health_check,
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
    let state = match AppState::new(&data_dir).await {
        Ok(state) => state,
        Err(e) => {
            tracing::error!("Error inicializando el estado de la aplicación: {}", e);
            std::process::exit(1);
        }
    };

    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://localhost:3000".to_string());
    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let cors = if origins.is_empty() {
        CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Rutas públicas de la API
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(get_church_info))
        .route("/schedule", get(get_schedule))
        .route("/prayer-request", post(create_prayer_request).get(get_prayer_requests))
        .layer(cors.clone());

    // Rutas de administración (con middleware de auth)
    let admin_routes = Router::new()
        .route("/login", post(admin_login))
        .route("/change-password", post(admin_change_password))
        .route("/events", get(admin_get_events).post(admin_create_event))
        .route("/events/:id", put(admin_update_event).delete(admin_delete_event))
        .route("/gallery", get(admin_get_gallery).post(admin_upload_gallery))
        .route("/gallery/:id", delete(admin_delete_gallery_item))
        .route("/export/events", get(admin_export_events_json))
        .route("/export/events/csv", get(admin_export_events_csv))
        .route("/export/prayers", get(admin_export_prayers_json))
        .route("/export/gallery", get(admin_export_gallery_json))
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware))
        .layer(cors);

    // Combinar API y admin bajo /api
    let combined_api = Router::new()
        .merge(api_routes)
        .merge(admin_routes)
        .with_state(state.clone());

    // Servir archivos subidos de la galería
    let uploads_dir = PathBuf::from(format!("{}/uploads", data_dir));

    // Servir archivos estáticos del frontend
    let dist_dir = PathBuf::from("../frontend/dist");
    let app = if dist_dir.exists() {
        tracing::info!("Servidor sirviendo archivos estáticos desde {:?}", dist_dir);
        Router::new()
            .nest("/api", combined_api)
            .nest_service("/uploads", ServeDir::new(&uploads_dir))
            .fallback_service(ServeDir::new(dist_dir))
            .with_state(state)
    } else {
        tracing::info!("Modo API: 'frontend/dist' no encontrado todavía, sirviendo solo API.");
        Router::new()
            .nest("/api", combined_api)
            .nest_service("/uploads", ServeDir::new(&uploads_dir))
            .route("/", get(|| async { "ICNVB API está en línea. Frontend en desarrollo o ejecutable en puerto independiente." }))
            .with_state(state)
    };

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(
        "🕊️ Servidor Iglesia Cristiana Nueva Vida corriendo en http://{}",
        addr
    );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
