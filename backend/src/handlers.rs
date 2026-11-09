use axum::{
    extract::{Query, State, Path},
    http::{StatusCode, HeaderMap, header},
    response::Json,
    routing::{get, post, put, delete},
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use tokio::fs;
use uuid::Uuid;

use crate::models::{
    ApiResponse, ApiResponseEmpty, ChurchInfo, CreateEventInput, CreateGalleryItemInput,
    CreatePrayerInput, GalleryItem, GalleryListResponse, LoginRequest, LoginResponse,
    MonthlyEvent, PrayerRequest, UpdateEventInput,
};
use crate::store::AppState;

// --- Middleware de autenticación simple ---

pub async fn auth_middleware(
    headers: HeaderMap,
    State(state): State<AppState>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let _ = state; // state disponible si se necesita en el futuro
    let path = request.uri().path().to_string();

    // Permitir login y health sin auth
    if path == "/api/admin/login" || path == "/api/health" {
        return Ok(next.run(request).await);
    }

    // Verificar token simple
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str == "Bearer admin-authenticated" {
                return Ok(next.run(request).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// --- Endpoints públicos existentes ---

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "ICNVB Backend",
        "church": "Iglesia Cristiana Nueva Vida - Bremen, Sincelejo"
    }))
}

pub async fn get_church_info(State(state): State<AppState>) -> Json<ApiResponse<ChurchInfo>> {
    let info = state.church_info.read().await.clone();
    Json(ApiResponse {
        success: true,
        message: "Información de la iglesia obtenida exitosamente".to_string(),
        data: Some(info),
    })
}

#[derive(Debug, Deserialize)]
pub struct ScheduleFilter {
    pub month: Option<String>,
    pub category: Option<String>,
}

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

    filtered.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));

    Json(ApiResponse {
        success: true,
        message: format!("Se encontraron {} eventos", filtered.len()),
        data: Some(filtered),
    })
}

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
        phone: payload.phone.map(|p| p.trim().to_string()).filter(|p| !p.is_empty()),
        category: payload.category.trim().to_string(),
        message: payload.message.trim().to_string(),
        created_at: Utc::now().format("%Y-%m-%d %H:%M").to_string(),
        is_private: payload.is_private.unwrap_or(false),
    };

    {
        let mut prayers = state.prayers.write().await;
        prayers.push(prayer.clone());
    }

    state.save_prayers_wrapper().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: "Tu petición de oración ha sido recibida. Los pastores y el equipo de intercesión estarán orando por ti.".to_string(),
            data: Some(prayer),
        }),
    ))
}

pub async fn get_prayer_requests(State(state): State<AppState>) -> Json<ApiResponse<Vec<PrayerRequest>>> {
    let prayers = state.prayers.read().await;
    let public_prayers: Vec<PrayerRequest> = prayers
        .iter()
        .filter(|p| !p.is_private)
        .cloned()
        .collect();

    Json(ApiResponse {
        success: true,
        message: "Peticiones de oración recuperadas".to_string(),
        data: Some(public_prayers),
    })
}

// --- Endpoints de Administración ---

pub async fn admin_login(
    State(state): State<AppState>,
    Json(login): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if state.verify_admin(&login.username, &login.password).await {
        Ok(Json(LoginResponse {
            token: "admin-authenticated".to_string(),
            user: crate::models::AdminUser {
                username: login.username,
                password_hash: "".to_string(),
            },
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

// GET /api/admin/events
pub async fn admin_get_events(State(state): State<AppState>) -> Json<ApiResponse<Vec<MonthlyEvent>>> {
    let events = state.events.read().await;
    let mut sorted: Vec<MonthlyEvent> = events.clone();
    sorted.sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));

    Json(ApiResponse {
        success: true,
        message: format!("Se encontraron {} eventos", sorted.len()),
        data: Some(sorted),
    })
}

// POST /api/admin/events
pub async fn admin_create_event(
    State(state): State<AppState>,
    Json(input): Json<CreateEventInput>,
) -> Result<(StatusCode, Json<ApiResponse<MonthlyEvent>>), StatusCode> {
    match state.add_event(input).await {
        Ok(event) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                message: "Evento creado exitosamente".to_string(),
                data: Some(event),
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// PUT /api/admin/events/:id
pub async fn admin_update_event(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<UpdateEventInput>,
) -> Result<Json<ApiResponse<MonthlyEvent>>, StatusCode> {
    match state.update_event(&id, input).await {
        Ok(Some(event)) => Ok(Json(ApiResponse {
            success: true,
            message: "Evento actualizado exitosamente".to_string(),
            data: Some(event),
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /api/admin/events/:id
pub async fn admin_delete_event(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponseEmpty>, StatusCode> {
    match state.delete_event(&id).await {
        Ok(true) => Ok(Json(ApiResponseEmpty {
            success: true,
            message: "Evento eliminado exitosamente".to_string(),
        })),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// GET /api/admin/gallery
pub async fn admin_get_gallery(State(state): State<AppState>) -> Json<GalleryListResponse> {
    let items = state.get_gallery_items().await;
    Json(GalleryListResponse {
        success: true,
        message: format!("Se encontraron {} elementos en la galería", items.len()),
        data: items,
    })
}

// POST /api/admin/gallery - subir archivo
pub async fn admin_upload_gallery(
    State(state): State<AppState>,
    mut multipart: axum::extract::Multipart,
) -> Result<(StatusCode, Json<ApiResponse<GalleryItem>>), StatusCode> {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut file_type: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut category: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "title" => title = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "description" => description = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "file_type" => file_type = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "file_name" => file_name = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "category" => category = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "file" => {
                file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec());
            }
            _ => {}
        }
    }

    let title = title.ok_or(StatusCode::BAD_REQUEST)?;
    let file_type = file_type.ok_or(StatusCode::BAD_REQUEST)?;
    let file_name = file_name.ok_or(StatusCode::BAD_REQUEST)?;
    let category = category.ok_or(StatusCode::BAD_REQUEST)?;
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;

    // Obtener data_dir mediante un método público o hacer el campo accesible
    let data_dir = state.data_dir.clone();

    // Guardar archivo físico
    let file_id = Uuid::new_v4().simple();
    let safe_file_name = file_name.replace('/', "_").replace('\\', "_");
    let file_path_on_disk = format!("{}/uploads/{}_{}", data_dir, file_id, safe_file_name);
    let file_url_for_web = format!("/uploads/{}_{}", file_id, safe_file_name);

    fs::write(&file_path_on_disk, &file_data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let input = CreateGalleryItemInput {
        title,
        description,
        file_type,
        file_name,
        category,
    };

    match state.add_gallery_item(input, file_url_for_web).await {
        Ok(item) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                message: "Archivo subido a la galería exitosamente".to_string(),
                data: Some(item),
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// DELETE /api/admin/gallery/:id
pub async fn admin_delete_gallery_item(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponseEmpty>, StatusCode> {
    match state.delete_gallery_item(&id).await {
        Ok(true) => Ok(Json(ApiResponseEmpty {
            success: true,
            message: "Elemento de galería eliminado exitosamente".to_string(),
        })),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
