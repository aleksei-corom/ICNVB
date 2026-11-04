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

