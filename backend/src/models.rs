use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChurchInfo {
    pub name: String,
    pub short_name: String,
    pub slogan: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pastors: Vec<String>,
    pub phones: Vec<String>,
    pub whatsapp_numbers: Vec<String>,
    pub regular_schedule: Vec<WeeklyScheduleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyScheduleItem {
    pub day: String,
    pub time: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyEvent {
    pub id: String,
    pub title: String,
    pub date: String, // Formato YYYY-MM-DD
    pub time: String,
    pub category: String, // "Culto General", "Jóvenes", "Damas", "Ayuno", "Vigilia", "Especial", "Grupos Familiares"
    pub description: String,
    pub scripture: Option<String>,
    pub predicador: Option<String>, // Nombre del predicador / expositor del culto
    pub director_alabanza: Option<String>, // Nombre del director de alabanza / ministerio de música
    pub is_featured: bool,
    pub location: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEventInput {
    pub title: String,
    pub date: String,
    pub time: String,
    pub category: String,
    pub description: String,
    pub scripture: Option<String>,
    pub predicador: Option<String>,
    pub director_alabanza: Option<String>,
    pub is_featured: Option<bool>,
    pub location: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEventInput {
    pub title: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub scripture: Option<Option<String>>,
    pub predicador: Option<Option<String>>,
    pub director_alabanza: Option<Option<String>>,
    pub is_featured: Option<bool>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerRequest {
    pub id: String,
    pub name: String,
    pub phone: Option<String>,
    pub category: String, // "Salud", "Familia", "Finanzas", "Espiritual", "Gratitud", "Otro"
    pub message: String,
    pub created_at: String,
    pub is_private: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePrayerInput {
    pub name: String,
    pub phone: Option<String>,
    pub category: String,
    pub message: String,
    pub is_private: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

// --- Galería ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub file_type: String, // "image" o "video"
    pub file_url: String,  // ruta relativa al archivo subido
    pub file_name: String,
    pub category: String, // "Culto", "Evento Especial", "Grupos Familiares", "Niños", "Jóvenes", "Otro"
    pub uploaded_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGalleryItemInput {
    pub title: String,
    pub description: Option<String>,
    pub file_type: String,
    pub file_name: String,
    pub category: String,
}

// --- Autenticación Admin ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub username: String,
    pub password_hash: String, // En producción usaría bcrypt/argon2
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: AdminUser,
}

#[derive(Debug, Serialize)]
pub struct ApiResponseEmpty {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GalleryListResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<GalleryItem>,
}

// --- JWT Claims ---

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub exp: usize,  // expiración
}

// --- Cambio de contraseña ---

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

// --- Exportación ---

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub success: bool,
    pub message: String,
    pub filename: String,
    pub content_type: String,
    pub data: String,
}
