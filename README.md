# 🕊️ Iglesia Cristiana Nueva Vida (ICNVB)
**Corregimiento Bremen, Municipio de Sincelejo, Sucre, Colombia**

> *"Un lugar para creer, pertenecer y crecer en Cristo"*  
> *"Yo me alegré con los que me decían: A la casa de Jehová iremos." — Salmos 122:1*

Sitio Web Oficial, moderno, profesional y reactivo para la Iglesia Cristiana Nueva Vida.

---

## 🌟 Características Principales

1. **Programación Mensual Interactiva**:
   - Visualización por mes (Noviembre, Diciembre, Enero, etc.).
   - Filtros por categoría: Cultos Generales, Jóvenes, Damas, Ayunos, Vigilias, Estudios Bíblicos.
   - Botón directo para **Compartir por WhatsApp** a familiares y amigos con un solo clic.
   - Botón para **Añadir a Google Calendar** con recordatorio.
2. **Horarios de Cultos Semanales**:
   - **Domingos**: 9:00 AM – 11:30 AM (Culto General & Escuela Bíblica Infantil).
   - **Miércoles**: 7:00 PM – 9:00 PM (Culto de Oración y Doctrina Bíblica).
   - **Jueves**: 9:00 AM – 12:00 M (Ayuno Congregacional de Intercesión).
   - **Jueves**: 7:00 PM – 9:00 PM (Culto de Damas - Mujeres de Fe).
   - **Viernes**: 7:00 PM – 9:00 PM (Culto de Jóvenes y Jovencitos).
   - **Vigilias**: Programación mensual anunciada en el calendario.
3. **Pastores y Liderazgo**:
   - Pastor Benjamín Ramos (+57 311 327 4403).
   - Pastora Martha Escudero (+57 313 509 4547).
   - Botones rápidos de llamada telefónica y chat de WhatsApp con un toque.
4. **Ministerios**:
   - Jóvenes y Jovencitos, Damas, Escuela Bíblica Infantil, Ayuno & Intercesión, Alabanza y Evangelismo.
5. **Peticiones de Oración**:
   - Formulario confidencial conectado a la API de Rust y opción para enviar la petición directo a los pastores por WhatsApp.
6. **Ubicación y Guía para Visitantes**:
   - Mapa interactivo de Bremen, Sincelejo.
   - Preguntas frecuentes para personas que asisten por primera vez.

---

## 🛠️ Arquitectura Tecnológica

- **Backend (API REST & Servidor de Alta Eficiencia)**:
  - Lenguaje: **Rust**
  - Framework Web: **Axum** con **Tokio** runtime asíncrono
  - Serialización: **Serde** & **serde_json**
  - Servidor estático integrado: `tower-http` sirve la web directamente en el puerto 3000
- **Frontend**:
  - Framework: **Svelte** con **Vite** & **TypeScript**
  - Estilos: **Tailwind CSS** (Paleta personalizada Azul Marino, Oro Cálido y Blanco)
  - Iconos: **Lucide Svelte**

---

## 🚀 Cómo Ejecutar el Proyecto

### Opción 1: Ejecutar la Solución Completa con Rust (Recomendado)

El servidor Rust compila y sirve tanto la API como la aplicación web en un solo proceso ultra-ligero:

```bash
# 1. Compilar el frontend (si realizas cambios en la interfaz)
cd frontend
npm run build

# 2. Iniciar el servidor Rust
cd ../backend
cargo run
```

Abre tu navegador en: **`http://localhost:3000`**

---

### Opción 2: Modo Desarrollo Frontend (con Hot-Reload instantáneo)

Para modificar el diseño y ver los cambios en tiempo real:

```bash
cd frontend
npm run dev
```

Abre tu navegador en: **`http://localhost:5173`** (Vite redirige automáticamente las llamadas de `/api` a Rust).

---

## 📅 Cómo Actualizar la Programación del Mes

Los eventos se almacenan en formato JSON en el archivo:
📁 **`data/schedule.json`**

Cada evento tiene la siguiente estructura:
```json
{
  "id": "evt-nov-01",
  "title": "Culto de Celebración y Santa Cena",
  "date": "2026-11-01",
  "time": "9:00 AM - 11:30 AM",
  "category": "Culto General",
  "description": "Conmemoramos el sacrificio de Cristo con la Santa Cena.",
  "scripture": "1 Corintios 11:24-26",
  "is_featured": true,
  "location": "Templo Central ICNVB - Bremen"
}
```

Al editar este archivo, el sitio web reflejará inmediatamente las nuevas fechas.

---

## 📱 Opciones de Despliegue en Internet (Hosting)

1. **Frontend en Vercel, Netlify o GitHub Pages**:
   - La carpeta `frontend/dist` contiene la web 100% estática optimizada que puedes arrastrar o conectar a GitHub para tener el sitio en línea gratis en segundos.
2. **VPS o Servidor en la Nube (Render, Railway, Fly.io, DigitalOcean)**:
   - Con el binario de Rust (`cargo build --release`), obtienes un ejecutable único que consume menos de 15 MB de RAM.

