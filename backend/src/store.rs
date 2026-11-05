use crate::models::{ChurchInfo, MonthlyEvent, PrayerRequest, WeeklyScheduleItem};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub church_info: Arc<RwLock<ChurchInfo>>,
    pub events: Arc<RwLock<Vec<MonthlyEvent>>>,
    pub prayers: Arc<RwLock<Vec<PrayerRequest>>>,
    data_dir: String,
}

impl AppState {
    pub async fn new(data_dir: &str) -> Result<Self, String> {
        fs::create_dir_all(data_dir).await.map_err(|e| {
            format!(
                "No se pudo crear el directorio de datos '{}': {}",
                data_dir, e
            )
        })?;

        let church_info = Self::load_or_default_info(data_dir).await;
        let events = Self::load_or_default_events(data_dir).await;
        let prayers = Self::load_prayers(data_dir).await;

        Ok(Self {
            church_info: Arc::new(RwLock::new(church_info)),
            events: Arc::new(RwLock::new(events)),
            prayers: Arc::new(RwLock::new(prayers)),
            data_dir: data_dir.to_string(),
        })
    }

    async fn load_or_default_info(dir: &str) -> ChurchInfo {
        let path = format!("{}/church_info.json", dir);
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(info) = serde_json::from_str(&content) {
                return info;
            }
        }

        let default_info = Self::default_church_info();

        if let Ok(serialized) = serde_json::to_string_pretty(&default_info) {
            let _ = fs::write(&path, serialized).await;
        }

        default_info
    }

    fn default_church_info() -> ChurchInfo {
        ChurchInfo {
            name: "Iglesia Cristiana Nueva Vida".to_string(),
            short_name: "ICNVB".to_string(),
            slogan: "Un lugar para creer, pertenecer y crecer en Cristo".to_string(),
            address: "Corregimiento Bremen".to_string(),
            city: "Sincelejo".to_string(),
            state: "Sucre".to_string(),
            country: "Colombia".to_string(),
            pastors: vec![
                "Pastor Benjamín Ramos".to_string(),
                "Pastora Martha Escudero".to_string(),
            ],
            phones: vec![
                "+57 311 327 4403".to_string(),
                "+57 313 509 4547".to_string(),
            ],
            whatsapp_numbers: vec![
                "573113274403".to_string(),
                "573135094547".to_string(),
            ],
            regular_schedule: vec![
                WeeklyScheduleItem {
                    day: "Domingos".to_string(),
                    time: "9:00 AM - 11:30 AM".to_string(),
                    title: "Culto General & Escuela Dominical".to_string(),
                    category: "Culto General".to_string(),
                    description: "Tiempo de alabanza, adoración congregacional, predicación de la Palabra de Dios y clases formativas para niños de todas las edades.".to_string(),
                    icon: "Sun".to_string(),
                },
                WeeklyScheduleItem {
                    day: "Miércoles".to_string(),
                    time: "7:00 PM - 9:00 PM".to_string(),
                    title: "Culto de Oración y Doctrina Bíblica".to_string(),
                    category: "Enseñanza".to_string(),
                    description: "Encuentro de fortalecimiento espiritual, estudio bíblico profundo y clamor unido por la comunidad y las familias.".to_string(),
                    icon: "BookOpen".to_string(),
                },
                WeeklyScheduleItem {
                    day: "Jueves (Mañana)".to_string(),
                    time: "9:00 AM - 12:00 M".to_string(),
                    title: "Ayuno Congregacional de Intercesión".to_string(),
                    category: "Ayuno".to_string(),
                    description: "Búsqueda del rostro de Dios en humillación, intercesión por los enfermos, quebrantamiento y renovación espiritual.".to_string(),
                    icon: "HeartHandshake".to_string(),
                },
                WeeklyScheduleItem {
                    day: "Jueves (Noche)".to_string(),
                    time: "7:00 PM - 9:00 PM".to_string(),
                    title: "Culto de Damas (Mujeres de Fe)".to_string(),
                    category: "Damas".to_string(),
                    description: "Reunión edificante para mujeres, madres e hijas, cultivando el testimonio cristiano, la hermandad y el poder de la oración.".to_string(),
                    icon: "Sparkles".to_string(),
                },
                WeeklyScheduleItem {
                    day: "Viernes".to_string(),
                    time: "7:00 PM - 9:00 PM".to_string(),
                    title: "Culto de Jóvenes y Jovencitos".to_string(),
                    category: "Jóvenes".to_string(),
                    description: "Espacio dinámico para la juventud con música, desafíos bíblicos, dinámicas e inspiración para vivir con propósito en Cristo.".to_string(),
                    icon: "Flame".to_string(),
                },
                WeeklyScheduleItem {
                    day: "Mensual".to_string(),
                    time: "Programación Mensual".to_string(),
                    title: "Gran Vigilia de Poder & Avivamiento".to_string(),
                    category: "Vigilia".to_string(),
                    description: "Noches especiales de búsqueda continua, vigilias de oración y adoración extendida anunciadas mes a mes en nuestro calendario.".to_string(),
                    icon: "Moon".to_string(),
                },
            ],
        }
    }

    async fn load_or_default_events(dir: &str) -> Vec<MonthlyEvent> {
        let path = format!("{}/schedule.json", dir);
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(events) = serde_json::from_str(&content) {
                return events;
            }
        }

        let default_events = Self::default_events();

        if let Ok(serialized) = serde_json::to_string_pretty(&default_events) {
            let _ = fs::write(&path, serialized).await;
        }

        default_events
    }

    fn default_events() -> Vec<MonthlyEvent> {
        vec![
            MonthlyEvent {
                id: "evt-nov-01".to_string(),
                title: "Culto de Celebración y Santa Cena".to_string(),
                date: "2026-11-01".to_string(),
                time: "9:00 AM - 11:30 AM".to_string(),
                category: "Culto General".to_string(),
                description: "Primer domingo del mes: Conmemoramos el sacrificio de Cristo en la cruz con la Santa Cena y acción de gracias comunitaria.".to_string(),
                scripture: Some("1 Corintios 11:24-26".to_string()),
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-04".to_string(),
                title: "Estudio Bíblico: Fundamentos de la Fe".to_string(),
                date: "2026-11-04".to_string(),
                time: "7:00 PM - 9:00 PM".to_string(),
                category: "Enseñanza".to_string(),
                description: "Serie doctrinal para profundizar en las promesas y mandamientos de Dios para la familia cristiana.".to_string(),
                scripture: Some("2 Timoteo 3:16-17".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-05-am".to_string(),
                title: "Mañana de Ayuno e Intercesión por las Familias".to_string(),
                date: "2026-11-05".to_string(),
                time: "9:00 AM - 12:00 M".to_string(),
                category: "Ayuno".to_string(),
                description: "Clamor por sanidad de los enfermos, restauración de hogares y cobertura espiritual sobre el corregimiento de Bremen.".to_string(),
                scripture: Some("Esdras 8:23".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-05-pm".to_string(),
                title: "Reunión de Damas: Valientes y Esforzadas".to_string(),
                date: "2026-11-05".to_string(),
                time: "7:00 PM - 9:00 PM".to_string(),
                category: "Damas".to_string(),
                description: "Culto especial para mujeres con alabanza y una palabra de fe y ánimo a cargo de la Pastora Martha Escudero.".to_string(),
                scripture: Some("Proverbios 31:25-30".to_string()),
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-06".to_string(),
                title: "Encuentro de Jóvenes y Jovencitos: Generación Sin Límites".to_string(),
                date: "2026-11-06".to_string(),
                time: "7:00 PM - 9:00 PM".to_string(),
                category: "Jóvenes".to_string(),
                description: "Alabanza contemporánea, testimonios y mensaje desafiante para la juventud. ¡No te quedes en casa!".to_string(),
                scripture: Some("1 Timoteo 4:12".to_string()),
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-08".to_string(),
                title: "Culto Dominical de Familia y Amigos".to_string(),
                date: "2026-11-08".to_string(),
                time: "9:00 AM - 11:30 AM".to_string(),
                category: "Culto General".to_string(),
                description: "Trae a tu familia y vecinos a escuchar un mensaje de esperanza y vida eterna.".to_string(),
                scripture: Some("Josué 24:15".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-12-am".to_string(),
                title: "Ayuno Congregacional de Sanidad".to_string(),
                date: "2026-11-12".to_string(),
                time: "9:00 AM - 12:00 M".to_string(),
                category: "Ayuno".to_string(),
                description: "Oración unida creyendo en el poder de Dios para sanar cuerpos y sanar almas.".to_string(),
                scripture: Some("Santiago 5:14-15".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-12-pm".to_string(),
                title: "Culto de Damas: Edificando el Hogar".to_string(),
                date: "2026-11-12".to_string(),
                time: "7:00 PM - 9:00 PM".to_string(),
                category: "Damas".to_string(),
                description: "Estudio práctico para damas sobre sabiduría bíblica en el hogar.".to_string(),
                scripture: Some("Proverbios 14:1".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-13".to_string(),
                title: "Noche de Jóvenes: Pasión por Su Presencia".to_string(),
                date: "2026-11-13".to_string(),
                time: "7:00 PM - 9:00 PM".to_string(),
                category: "Jóvenes".to_string(),
                description: "Tiempo especial de adoración íntima y reconciliación espiritual para jóvenes y adolescentes.".to_string(),
                scripture: Some("Salmos 119:9".to_string()),
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-20".to_string(),
                title: "Gran Vigilia Mensual Unida: Clamor por Sucre y Bremen".to_string(),
                date: "2026-11-20".to_string(),
                time: "8:00 PM - 1:00 AM".to_string(),
                category: "Vigilia".to_string(),
                description: "Noche extraordinaria de oración, intercesión continua, predicación y búsqueda intensa del Espíritu Santo.".to_string(),
                scripture: Some("Lucas 6:12; Jeremías 33:3".to_string()),
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-29".to_string(),
                title: "Culto Especial de Acción de Gracias & Cierre de Mes".to_string(),
                date: "2026-11-29".to_string(),
                time: "9:00 AM - 11:30 AM".to_string(),
                category: "Culto General".to_string(),
                description: "Damos gracias a Dios por sus maravillas durante todo el mes con testimonios y alabanza con toda la congregación.".to_string(),
                scripture: Some("Salmos 100:1-5".to_string()),
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
        ]
    }

    async fn load_prayers(dir: &str) -> Vec<PrayerRequest> {
        let path = format!("{}/prayers.json", dir);
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(prayers) = serde_json::from_str(&content) {
                return prayers;
            }
        }
        Vec::new()
    }

    pub async fn save_events(&self) -> Result<(), String> {
        let path = format!("{}/schedule.json", self.data_dir);
        let events = self.events.read().await;
        let serialized = serde_json::to_string_pretty(&*events)
            .map_err(|e| format!("Error serializando eventos: {}", e))?;
        fs::write(&path, serialized)
            .await
            .map_err(|e| format!("Error guardando eventos en disco: {}", e))?;
        Ok(())
    }

    pub async fn save_prayers(&self) -> Result<(), String> {
        let path = format!("{}/prayers.json", self.data_dir);
        let prayers = self.prayers.read().await;
        let serialized = serde_json::to_string_pretty(&*prayers)
            .map_err(|e| format!("Error serializando peticiones: {}", e))?;
        fs::write(&path, serialized)
            .await
            .map_err(|e| format!("Error guardando peticiones en disco: {}", e))?;
        Ok(())
    }
}
