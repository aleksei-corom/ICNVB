export interface WeeklyScheduleItem {
  day: string;
  time: string;
  title: string;
  category: string;
  description: string;
  icon: string;
}

export interface MonthlyEvent {
  id: string;
  title: string;
  date: string; // YYYY-MM-DD
  time: string;
  category: string;
  description: string;
  scripture?: string;
  predicador?: string; // Nombre del predicador / expositor del culto
  directorAlabanza?: string; // Nombre del director de alabanza / ministerio de música
  is_featured: boolean;
  location: string;
}

export interface ChurchInfo {
  name: string;
  short_name: string;
  slogan: string;
  address: string;
  city: string;
  state: string;
  country: string;
  pastors: string[];
  phones: string[];
  whatsapp_numbers: string[];
  regular_schedule: WeeklyScheduleItem[];
}

export interface PrayerInput {
  name: string;
  phone?: string;
  category: string;
  message: string;
  is_private: boolean;
}

export interface GalleryItem {
  id: string;
  title: string;
  description?: string;
  file_type: string; // "image" o "video"
  file_url: string;
  file_name: string;
  category: string;
  uploaded_at: string;
}

export interface AdminUser {
  username: string;
  password_hash: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: AdminUser;
}

export interface CreateEventInput {
  title: string;
  date: string;
  time: string;
  category: string;
  description: string;
  scripture?: string;
  predicador?: string;
  director_alabanza?: string;
  is_featured?: boolean;
  location: string;
}

export interface UpdateEventInput {
  title?: string;
  date?: string;
  time?: string;
  category?: string;
  description?: string;
  scripture?: string | null;
  predicador?: string | null;
  director_alabanza?: string | null;
  is_featured?: boolean;
  location?: string;
}

