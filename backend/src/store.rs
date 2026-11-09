use crate::models::{
    AdminUser, ChurchInfo, CreateEventInput, CreateGalleryItemInput, GalleryItem,
    MonthlyEvent, PrayerRequest, UpdateEventInput, WeeklyScheduleItem,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Clone)]
pub struct AppState {
    pub church_info: Arc<RwLock<ChurchInfo>>,
    pub events: Arc<RwLock<Vec<MonthlyEvent>>>,
    pub prayers: Arc<RwLock<Vec<PrayerRequest>>>,
    pub gallery: Arc<RwLock<Vec<GalleryItem>>>,
    pub admin_user: Arc<RwLock<AdminUser>>,
    pub data_dir: String,
}

impl AppState {
    pub async fn new(data_dir: &str) -> Result<Self, String> {
        fs::create_dir_all(data_dir).await.map_err(|e| format!("No se pudo crear el directorio de datos '{}': {}", data_dir, e))?;

        // Crear subdirectorio para uploads
        let uploads_dir = format!("{}/uploads", data_dir);
        fs::create_dir_all(&uploads_dir).await.map_err(|e| format!("No se pudo crear el directorio de uploads: {}", e))?;

        let church_info = Self::load_or_default_info(data_dir).await;
        let events = Self::load_or_default_events(data_dir).await;
        let prayers = Self::load_prayers(data_dir).await;
        let gallery = Self::load_gallery(data_dir).await;
        let admin_user = Self::load_or_default_admin(data_dir).await;

        Ok(Self {
            church_info: Arc::new(RwLock::new(church_info)),
            events: Arc::new(RwLock::new(events)),
            prayers: Arc::new(RwLock::new(prayers)),
            gallery: Arc::new(RwLock::new(gallery)),
            admin_user: Arc::new(RwLock::new(admin_user)),
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
                is_featured: false,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-16-las-flores".to_string(),
                title: "Grupo Familiar - Las Flores".to_string(),
                date: "2026-11-16".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Familia Monterroza.".to_string(),
                scripture: None,
                predicador: Some("Manuel Acosta".to_string()),
                director_alabanza: Some("Consuelo García".to_string()),
                is_featured: false,
                location: "Casa de la Familia Monterroza - Las Flores".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-16-bremen-ruby".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-11-16".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Ruby.".to_string(),
                scripture: None,
                predicador: Some("Mariela Campos".to_string()),
                director_alabanza: Some("Luz Marina".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Ruby - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-16-bremen-catalina".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-11-16".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Catalina.".to_string(),
                scripture: None,
                predicador: Some("Carmelo Contreras".to_string()),
                director_alabanza: Some("Margelis y Derli".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Catalina - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-nov-20".to_string(),
                title: "Gran Vigilia Mensual Unida: Clamor por Sucre y Bremen".to_string(),
                date: "2026-11-20".to_string(),
                time: "8:00 PM - 1:00 AM".to_string(),
                category: "Vigilia".to_string(),
                description: "Noche extraordinaria de oración, intercesión continua, predicación y búsqueda intensa del Espíritu Santo.".to_string(),
                scripture: Some("Lucas 6:12; Jeremías 33:3".to_string()),
                predicador: None,
                director_alabanza: None,
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
                predicador: None,
                director_alabanza: None,
                is_featured: true,
                location: "Templo Central ICNVB - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-07-bremen-margelis".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-12-07".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Margelis.".to_string(),
                scripture: None,
                predicador: Some("Manuel Acosta".to_string()),
                director_alabanza: Some("Derli Bustamante".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Margelis - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-07-bremen-ruby".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-12-07".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Ruby.".to_string(),
                scripture: None,
                predicador: Some("Deimer Padilla".to_string()),
                director_alabanza: Some("Evangelina".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Ruby - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-07-las-flores-yuli".to_string(),
                title: "Grupo Familiar - Las Flores".to_string(),
                date: "2026-12-07".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de Yuli Contreras.".to_string(),
                scripture: None,
                predicador: Some("Elvira Méndez".to_string()),
                director_alabanza: Some("Consuelo García".to_string()),
                is_featured: false,
                location: "Casa de Yuli Contreras - Las Flores".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-22-bremen-ruby".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-12-22".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Ruby.".to_string(),
                scripture: None,
                predicador: Some("Eustorguio Jaraba".to_string()),
                director_alabanza: Some("Maylin".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Ruby - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-22-bremen-catalina".to_string(),
                title: "Grupo Familiar - Bremen".to_string(),
                date: "2026-12-22".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Hermana Catalina.".to_string(),
                scripture: None,
                predicador: Some("Margelis Peres".to_string()),
                director_alabanza: Some("Derli Bustamante".to_string()),
                is_featured: false,
                location: "Casa de la Hermana Catalina - Bremen".to_string(),
            },
            MonthlyEvent {
                id: "evt-dec-22-las-flores-monteroza".to_string(),
                title: "Grupo Familiar - Las Flores".to_string(),
                date: "2026-12-22".to_string(),
                time: "7:00 PM".to_string(),
                category: "Grupos Familiares".to_string(),
                description: "Culto familiar en casa de la Familia Monteroza.".to_string(),
                scripture: None,
                predicador: Some("Mariela Campo".to_string()),
                director_alabanza: Some("Consuelo García".to_string()),
                is_featured: false,
                location: "Casa de la Familia Monteroza - Las Flores".to_string(),
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

    async fn load_gallery(dir: &str) -> Vec<GalleryItem> {
        let path = format!("{}/gallery.json", dir);
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(items) = serde_json::from_str(&content) {
                return items;
            }
        }
        Vec::new()
    }

    async fn load_or_default_admin(dir: &str) -> AdminUser {
        let path = format!("{}/admin.json", dir);
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(user) = serde_json::from_str(&content) {
                return user;
            }
        }

        // Usuario por defecto: admin / icnvb2026 (con hash bcrypt)
        let password_hash = hash("icnvb2026", DEFAULT_COST).unwrap_or_else(|_| "icnvb2026".to_string());

        let default_user = AdminUser {
            username: "admin".to_string(),
            password_hash,
        };

        if let Ok(serialized) = serde_json::to_string_pretty(&default_user) {
            let _ = fs::write(&path, serialized).await;
        }

        default_user
    }

    // --- Eventos CRUD ---

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

    pub async fn add_event(&self, input: CreateEventInput) -> Result<MonthlyEvent, String> {
        let event = MonthlyEvent {
            id: format!("evt-{}", Uuid::new_v4().simple()),
            title: input.title.trim().to_string(),
            date: input.date.trim().to_string(),
            time: input.time.trim().to_string(),
            category: input.category.trim().to_string(),
            description: input.description.trim().to_string(),
            scripture: input.scripture.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            predicador: input.predicador.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            director_alabanza: input.director_alabanza.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            is_featured: input.is_featured.unwrap_or(false),
            location: input.location.trim().to_string(),
        };

        {
            let mut events = self.events.write().await;
            events.push(event.clone());
        }

        self.save_events().await?;

        Ok(event)
    }

    pub async fn update_event(&self, id: &str, input: UpdateEventInput) -> Result<Option<MonthlyEvent>, String> {
        let mut events = self.events.write().await;
        let event_index = events.iter().position(|e| e.id == id);

        match event_index {
            Some(idx) => {
                let event = &mut events[idx];
                if let Some(title) = input.title { event.title = title.trim().to_string(); }
                if let Some(date) = input.date { event.date = date.trim().to_string(); }
                if let Some(time) = input.time { event.time = time.trim().to_string(); }
                if let Some(category) = input.category { event.category = category.trim().to_string(); }
                if let Some(description) = input.description { event.description = description.trim().to_string(); }
                if let Some(scripture) = input.scripture { event.scripture = scripture.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()); }
                if let Some(predicador) = input.predicador { event.predicador = predicador.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()); }
                if let Some(director_alabanza) = input.director_alabanza { event.director_alabanza = director_alabanza.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()); }
                if let Some(is_featured) = input.is_featured { event.is_featured = is_featured; }
                if let Some(location) = input.location { event.location = location.trim().to_string(); }

                let updated_event = event.clone();
                drop(events); // liberar lock antes de guardar
                self.save_events().await?;

                Ok(Some(updated_event))
            }
            None => Ok(None),
        }
    }

    pub async fn delete_event(&self, id: &str) -> Result<bool, String> {
        let mut events = self.events.write().await;
        let initial_len = events.len();
        events.retain(|e| e.id != id);

        if events.len() < initial_len {
            drop(events);
            self.save_events().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // --- Peticiones de oración ---

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

    // --- Galería ---

    pub async fn save_gallery(&self) -> Result<(), String> {
        let path = format!("{}/gallery.json", self.data_dir);
        let items = self.gallery.read().await;
        let serialized = serde_json::to_string_pretty(&*items)
            .map_err(|e| format!("Error serializando galería: {}", e))?;
        fs::write(&path, serialized)
            .await
            .map_err(|e| format!("Error guardando galería en disco: {}", e))?;
        Ok(())
    }

    pub async fn add_gallery_item(&self, input: CreateGalleryItemInput, file_path: String) -> Result<GalleryItem, String> {
        let item = GalleryItem {
            id: format!("gal-{}", Uuid::new_v4().simple()),
            title: input.title.trim().to_string(),
            description: input.description.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            file_type: input.file_type.trim().to_string(),
            file_url: file_path,
            file_name: input.file_name.trim().to_string(),
            category: input.category.trim().to_string(),
            uploaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        {
            let mut gallery = self.gallery.write().await;
            gallery.push(item.clone());
        }

        self.save_gallery().await?;

        Ok(item)
    }

    pub async fn delete_gallery_item(&self, id: &str) -> Result<bool, String> {
        let mut gallery = self.gallery.write().await;
        let item = gallery.iter().find(|g| g.id == id).cloned();

        if let Some(item) = item {
            // Intentar eliminar el archivo físico
            let file_path = format!("{}/{}", self.data_dir, item.file_url);
            let _ = fs::remove_file(&file_path).await;

            gallery.retain(|g| g.id != id);
            drop(gallery);
            self.save_gallery().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn get_gallery_items(&self) -> Vec<GalleryItem> {
        self.gallery.read().await.clone()
    }

    // --- Auth ---

    pub async fn verify_admin(&self, username: &str, password: &str) -> bool {
        let user = self.admin_user.read().await;
        if user.username != username {
            return false;
        }
        verify(password, &user.password_hash).unwrap_or(false)
    }

    pub async fn change_password(&self, username: &str, current_password: &str, new_password: &str) -> Result<(), String> {
        let mut user = self.admin_user.write().await;

        if user.username != username {
            return Err("Usuario no encontrado".to_string());
        }

        if !verify(current_password, &user.password_hash).unwrap_or(false) {
            return Err("Contraseña actual incorrecta".to_string());
        }

        let new_hash = hash(new_password, DEFAULT_COST)
            .map_err(|e| format!("Error hashing contraseña: {}", e))?;
        user.password_hash = new_hash;

        drop(user);
        self.save_admin().await
    }

    pub async fn save_admin(&self) -> Result<(), String> {
        let path = format!("{}/admin.json", self.data_dir);
        let user = self.admin_user.read().await;
        let serialized = serde_json::to_string_pretty(&*user)
            .map_err(|e| format!("Error serializando admin: {}", e))?;
        fs::write(&path, serialized)
            .await
            .map_err(|e| format!("Error guardando admin: {}", e))?;
        Ok(())
    }

    // --- Exportación de datos ---

    pub async fn export_events_json(&self) -> Result<String, String> {
        let events = self.events.read().await;
        let serialized = serde_json::to_string_pretty(&*events)
            .map_err(|e| format!("Error serializando eventos: {}", e))?;
        Ok(serialized)
    }

    pub async fn export_gallery_json(&self) -> Result<String, String> {
        let items = self.gallery.read().await;
        let serialized = serde_json::to_string_pretty(&*items)
            .map_err(|e| format!("Error serializando galería: {}", e))?;
        Ok(serialized)
    }

    pub async fn export_events_csv(&self) -> Result<String, String> {
        let events = self.events.read().await;
        let mut csv = String::from("id,title,date,time,category,description,scripture,predicador,director_alabanza,is_featured,location\n");
        for e in events.iter() {
            let scripture = e.scripture.as_deref().unwrap_or("");
            let predicador = e.predicador.as_deref().unwrap_or("");
            let director = e.director_alabanza.as_deref().unwrap_or("");
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{},\"{}\"\n",
                e.id, e.title, e.date, e.time, e.category, e.description,
                scripture, predicador, director, e.is_featured, e.location
            ));
        }
        Ok(csv)
    }

    pub async fn export_prayers_json(&self) -> Result<String, String> {
        let prayers = self.prayers.read().await;
        let serialized = serde_json::to_string_pretty(&*prayers)
            .map_err(|e| format!("Error serializando peticiones: {}", e))?;
        Ok(serialized)
    }

    pub async fn save_prayers_wrapper(&self) -> Result<(), String> {
        self.save_prayers().await
    }
}
