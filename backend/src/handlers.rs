use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{ApiResponse, ChurchInfo, CreatePrayerInput, MonthlyEvent, PrayerRequest};
use crate::store::AppState;

#[derive(Debug, Deserialize)]
pub struct ScheduleFilter {
    pub month: Option<String>,    // e.g. "2026-11"
    pub category: Option<String>, // e.g. "Jóvenes", "Damas"
}

// GET /api/info
pub async fn get_church_info(State(state): State<AppState>) -> Json<ApiResponse<ChurchInfo>> {
    let info = state.church_info.read().await.clone();
    Json(ApiResponse {
        success: true,
        message: "Información de la iglesia obtenida exitosamente".to_string(),
        data: Some(info),
    })
}

// GET /api/schedule
pub async fn get_schedule(
    Query(filter): Query<ScheduleFilter>,
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<MonthlyEvent>>> {
    let events = state.events.read().await;
    let mut filtered: Vec<MonthlyEvent> = events.clone();

    if let Some(month_prefix) = &filter.month {
        if !month_prefix.is_empty() {
            filtered.retain(|e| e.date.starts_with(month_prefix));
        }
    }

    if let Some(category) = &filter.category {
        if !category.is_empty() && category != "Todos" {
            filtered.retain(|e| e.category.eq_ignore_ascii_case(category));
        }
    }

    // Ordenar cronológicamente
    filtered.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));

    Json(ApiResponse {
        success: true,
        message: format!("Se encontraron {} eventos", filtered.len()),
        data: Some(filtered),
    })
}

// POST /api/schedule
pub async fn add_monthly_event(
    State(state): State<AppState>,
    Json(mut new_event): Json<MonthlyEvent>,
) -> Result<(StatusCode, Json<ApiResponse<MonthlyEvent>>), StatusCode> {
    if new_event.id.is_empty() {
        new_event.id = format!("evt-{}", Uuid::new_v4().simple());
    }

    {
        let mut events = state.events.write().await;
        events.push(new_event.clone());
    }

    state
        .save_events()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: "Evento agregado exitosamente a la programación".to_string(),
            data: Some(new_event),
        }),
    ))
}

// POST /api/prayer-request
pub async fn create_prayer_request(
    State(state): State<AppState>,
    Json(payload): Json<CreatePrayerInput>,
) -> Result<(StatusCode, Json<ApiResponse<PrayerRequest>>), StatusCode> {
    if payload.name.trim().is_empty() || payload.message.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let prayer = PrayerRequest {
        id: format!("pry-{}", Uuid::new_v4().simple()),
        name: payload.name.trim().to_string(),
        phone: payload
            .phone
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty()),
        category: payload.category.trim().to_string(),
        message: payload.message.trim().to_string(),
        created_at: Utc::now().format("%Y-%m-%d %H:%M").to_string(),
        is_private: payload.is_private.unwrap_or(false),
    };

    {
        let mut prayers = state.prayers.write().await;
        prayers.push(prayer.clone());
    }

    state
        .save_prayers()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: "Tu petición de oración ha sido recibida. Los pastores y el equipo de intercesión estarán orando por ti.".to_string(),
            data: Some(prayer),
        }),
    ))
}

// GET /api/prayer-requests (Peticiones públicas o testimonios)
pub async fn get_prayer_requests(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<PrayerRequest>>> {
    let prayers = state.prayers.read().await;
    // Excluir peticiones marcadas como privadas
    let public_prayers: Vec<PrayerRequest> =
        prayers.iter().filter(|p| !p.is_private).cloned().collect();

    Json(ApiResponse {
        success: true,
        message: "Peticiones de oración recuperadas".to_string(),
        data: Some(public_prayers),
    })
}

// GET /api/health
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "ICNVB Backend",
        "church": "Iglesia Cristiana Nueva Vida - Bremen, Sincelejo"
    }))
}
