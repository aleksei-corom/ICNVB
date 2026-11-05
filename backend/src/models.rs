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
    pub category: String, // "Culto General", "Jóvenes", "Damas", "Ayuno", "Vigilia", "Especial"
    pub description: String,
    pub scripture: Option<String>,
    pub is_featured: bool,
    pub location: String,
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
