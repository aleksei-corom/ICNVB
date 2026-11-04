<script lang="ts">
  import { onMount } from 'svelte';
  import { Calendar, Filter, Share2, Sparkles, MapPin, Clock, BookOpen, RefreshCw, FileSpreadsheet, ExternalLink, Info } from 'lucide-svelte';
  import type { MonthlyEvent } from '../types';
  import { APP_CONFIG } from '../config';
  import { fetchEventsFromGoogleSheets } from '../services/sheets';

  let events: MonthlyEvent[] = [];
  let loading = true;
  let isRefreshing = false;
  let isGoogleSheetsActive = Boolean(APP_CONFIG.googleSheetsCsvUrl && APP_CONFIG.googleSheetsCsvUrl.trim().length > 0);
  let showInstructionsModal = false;

  let selectedCategory = "Todos";
  let selectedMonth = "2026-11";

  const months = [
    { value: "2026-11", label: "Noviembre 2026" },
    { value: "2026-12", label: "Diciembre 2026" },
    { value: "2027-01", label: "Enero 2027" },
  ];

  const categories = [
    "Todos",
    "Culto General",
    "Jóvenes",
    "Damas",
    "Ayuno",
    "Vigilia",
    "Enseñanza",
  ];

  // Eventos de respaldo
  const defaultEvents: MonthlyEvent[] = [
    {
      id: "evt-nov-01",
      title: "Culto de Celebración y Santa Cena",
      date: "2026-11-01",
      time: "9:00 AM - 11:30 AM",
      category: "Culto General",
      description: "Primer domingo del mes: Conmemoramos el sacrificio de Cristo en la cruz con la Santa Cena y acción de gracias comunitaria.",
      scripture: "1 Corintios 11:24-26",
      is_featured: true,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-04",
      title: "Estudio Bíblico: Fundamentos de la Fe",
      date: "2026-11-04",
      time: "7:00 PM - 9:00 PM",
      category: "Enseñanza",
      description: "Serie doctrinal para profundizar en las promesas y mandamientos de Dios para la familia cristiana.",
      scripture: "2 Timoteo 3:16-17",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-05-am",
      title: "Mañana de Ayuno e Intercesión por las Familias",
      date: "2026-11-05",
      time: "9:00 AM - 12:00 M",
      category: "Ayuno",
      description: "Clamor por sanidad de los enfermos, restauración de hogares y cobertura espiritual sobre el corregimiento de Bremen.",
      scripture: "Esdras 8:23",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-05-pm",
      title: "Reunión de Damas: Valientes y Esforzadas",
      date: "2026-11-05",
      time: "7:00 PM - 9:00 PM",
      category: "Damas",
      description: "Culto especial para mujeres con alabanza y una palabra de fe y ánimo a cargo de la Pastora Martha Escudero.",
      scripture: "Proverbios 31:25-30",
      is_featured: true,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-06",
      title: "Encuentro de Jóvenes y Jovencitos: Generación Sin Límites",
      date: "2026-11-06",
      time: "7:00 PM - 9:00 PM",
      category: "Jóvenes",
      description: "Alabanza contemporánea, testimonios y mensaje desafiante para la juventud. ¡No te quedes en casa!",
      scripture: "1 Timoteo 4:12",
      is_featured: true,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-08",
      title: "Culto Dominical de Familia y Amigos",
      date: "2026-11-08",
      time: "9:00 AM - 11:30 AM",
      category: "Culto General",
      description: "Trae a tu familia y vecinos a escuchar un mensaje de esperanza y vida eterna.",
      scripture: "Josué 24:15",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-12-am",
      title: "Ayuno Congregacional de Sanidad",
      date: "2026-11-12",
      time: "9:00 AM - 12:00 M",
      category: "Ayuno",
      description: "Oración unida creyendo en el poder de Dios para sanar cuerpos y sanar almas.",
      scripture: "Santiago 5:14-15",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-12-pm",
      title: "Culto de Damas: Edificando el Hogar",
      date: "2026-11-12",
      time: "7:00 PM - 9:00 PM",
      category: "Damas",
      description: "Estudio práctico para damas sobre sabiduría bíblica en el hogar.",
      scripture: "Proverbios 14:1",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-13",
      title: "Noche de Jóvenes: Pasión por Su Presencia",
      date: "2026-11-13",
      time: "7:00 PM - 9:00 PM",
      category: "Jóvenes",
      description: "Tiempo especial de adoración íntima y reconciliación espiritual para jóvenes y adolescentes.",
      scripture: "Salmos 119:9",
      is_featured: false,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-20",
      title: "Gran Vigilia Mensual Unida: Clamor por Sucre y Bremen",
      date: "2026-11-20",
      time: "8:00 PM - 1:00 AM",
      category: "Vigilia",
      description: "Noche extraordinaria de oración, intercesión continua, predicación y búsqueda intensa del Espíritu Santo.",
      scripture: "Lucas 6:12; Jeremías 33:3",
      is_featured: true,
      location: "Templo Central ICNVB - Bremen",
    },
    {
      id: "evt-nov-29",
      title: "Culto Especial de Acción de Gracias & Cierre de Mes",
      date: "2026-11-29",
      time: "9:00 AM - 11:30 AM",
      category: "Culto General",
      description: "Damos gracias a Dios por sus maravillas durante todo el mes con testimonios y alabanza con toda la congregación.",
      scripture: "Salmos 100:1-5",
      is_featured: true,
      location: "Templo Central ICNVB - Bremen",
    },
  ];

  async function loadEvents() {
    loading = true;
    try {
      // 1. Intentar cargar desde Google Sheets si está configurado
      if (APP_CONFIG.googleSheetsCsvUrl && APP_CONFIG.googleSheetsCsvUrl.trim().length > 0) {
        const sheetEvents = await fetchEventsFromGoogleSheets(APP_CONFIG.googleSheetsCsvUrl);
        if (sheetEvents && sheetEvents.length > 0) {
          events = sheetEvents;
          isGoogleSheetsActive = true;
          return;
        }
      }

      // 2. Intentar cargar desde el backend en Rust (/api/schedule)
      const res = await fetch(`/api/schedule?month=${selectedMonth}`);
      if (res.ok) {
        const json = await res.json();
        if (json.success && json.data && json.data.length > 0) {
          events = json.data;
          return;
        }
      }

      // 3. Fallback a eventos predeterminados
      events = defaultEvents;
    } catch {
      events = defaultEvents;
    } finally {
      loading = false;
      isRefreshing = false;
    }
  }

  function handleRefresh() {
    isRefreshing = true;
    loadEvents();
  }

  onMount(() => {
    loadEvents();
  });

  // Filtrado reactivo por categoría y mes
  $: filteredEvents = events.filter(e => {
    const matchesMonth = e.date.startsWith(selectedMonth);
    if (!matchesMonth) return false;

    if (selectedCategory === "Todos") return true;
    return e.category.toLowerCase() === selectedCategory.toLowerCase();
  });

  function formatDateInfo(dateStr: string) {
    const parts = dateStr.split('-');
    const year = parseInt(parts[0], 10);
    const month = parseInt(parts[1], 10) - 1;
    const day = parseInt(parts[2], 10);
    const date = new Date(year, month, day);

    const dayName = date.toLocaleDateString('es-CO', { weekday: 'long' });
    const monthName = date.toLocaleDateString('es-CO', { month: 'short' });

    return {
      dayNumber: String(day).padStart(2, '0'),
      dayName: dayName.toUpperCase(),
      monthName: monthName.toUpperCase(),
    };
  }

  function shareEvent(event: MonthlyEvent) {
    const info = formatDateInfo(event.date);
    const text = `🕊️ *Iglesia Cristiana Nueva Vida - Bremen, Sincelejo*
📅 *${event.title}*
🗓️ *Fecha:* ${info.dayName} ${info.dayNumber} de ${info.monthName}
⏰ *Hora:* ${event.time}
📍 *Lugar:* ${event.location}
${event.scripture ? `📖 *Palabra:* ${event.scripture}` : ''}

${event.description}

¡Ven con toda tu familia! ¡Te esperamos!`;
    const url = `https://api.whatsapp.com/send?text=${encodeURIComponent(text)}`;
    window.open(url, '_blank');
  }

  function addToCalendar(event: MonthlyEvent) {
    const title = encodeURIComponent(`${event.title} - Iglesia Cristiana Nueva Vida`);
    const details = encodeURIComponent(`${event.description}\n\nLugar: ${event.location}\nBremen, Sincelejo`);
    const location = encodeURIComponent("Iglesia Cristiana Nueva Vida, Corregimiento Bremen, Sincelejo, Sucre");
    
    const dateFormatted = event.date.replace(/-/g, '');
    const gCalUrl = `https://calendar.google.com/calendar/render?action=TEMPLATE&text=${title}&details=${details}&location=${location}&dates=${dateFormatted}T140000Z/${dateFormatted}T163000Z`;
    window.open(gCalUrl, '_blank');
  }

  function getCategoryStyles(category: string) {
    switch (category.toLowerCase()) {
      case 'culto general':
        return 'bg-church-100 text-church-800 border-church-200';
      case 'jóvenes':
        return 'bg-amber-100 text-amber-800 border-amber-200';
      case 'damas':
        return 'bg-pink-100 text-pink-800 border-pink-200';
      case 'ayuno':
        return 'bg-emerald-100 text-emerald-800 border-emerald-200';
      case 'vigilia':
        return 'bg-indigo-900 text-indigo-100 border-indigo-700 font-bold';
      case 'enseñanza':
        return 'bg-sky-100 text-sky-800 border-sky-200';
      default:
        return 'bg-slate-100 text-slate-800 border-slate-200';
    }
  }
</script>

<section id="programacion" class="py-20 bg-white relative">
  <div class="absolute inset-0 bg-pattern pointer-events-none opacity-40"></div>

  <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <!-- Encabezado de la Sección -->
    <div class="text-center max-w-3xl mx-auto mb-10">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-gold-100 text-gold-900 text-xs font-bold uppercase tracking-wider mb-3 border border-gold-200">
        <Sparkles class="w-3.5 h-3.5 text-gold-600" />
        <span>Calendario y Actividades</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Programación Mensual de la Iglesia
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        Consulta las fechas, cultos especiales, vigilias y actividades programadas para este mes en Bremen, Sincelejo. ¡Organiza tu agenda y acompáñanos!
      </p>

      <!-- Estado de sincronización y botón para editar -->
      <div class="mt-4 flex flex-wrap items-center justify-center gap-3 text-xs">
        {#if isGoogleSheetsActive}
          <span class="inline-flex items-center gap-1 px-3 py-1 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 font-medium">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-ping"></span>
            <span>Sincronizado en tiempo real con Google Sheets</span>
          </span>
        {/if}

        <button
          type="button"
          on:click={handleRefresh}
          class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-slate-100 hover:bg-slate-200 text-slate-700 font-medium transition-colors"
          title="Recargar datos más recientes"
        >
          <RefreshCw class="w-3 h-3 {isRefreshing ? 'animate-spin text-church-600' : ''}" />
          <span>{isRefreshing ? 'Actualizando...' : 'Refrescar'}</span>
        </button>

        <button
          type="button"
          on:click={() => showInstructionsModal = true}
          class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-50 hover:bg-church-100 text-church-700 border border-church-200 font-medium transition-colors"
        >
          <FileSpreadsheet class="w-3.5 h-3.5 text-church-600" />
          <span>¿Cómo modificar eventos desde el celular?</span>
        </button>
      </div>
    </div>

    <!-- Barra de Control: Filtros y Selector de Mes -->
    <div class="bg-slate-50 border border-slate-200/90 rounded-2xl p-4 sm:p-6 mb-10 shadow-sm flex flex-col md:flex-row justify-between items-center gap-4">
      
      <!-- Selector de Mes -->
      <div class="flex items-center gap-3 w-full md:w-auto">
        <span class="text-xs uppercase font-bold text-slate-500 tracking-wider flex items-center gap-1">
          <Calendar class="w-4 h-4 text-church-600" />
          <span>Mes:</span>
        </span>
        <div class="flex bg-white p-1 rounded-xl border border-slate-200 shadow-inner overflow-x-auto">
          {#each months as m}
            <button
              type="button"
              class="px-3.5 py-1.5 rounded-lg text-xs sm:text-sm font-semibold transition-all whitespace-nowrap {selectedMonth === m.value ? 'bg-church-800 text-white shadow-sm' : 'text-slate-600 hover:text-slate-900 hover:bg-slate-100'}"
              on:click={() => selectedMonth = m.value}
            >
              {m.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Filtro por Categoría -->
      <div class="flex items-center gap-2 w-full md:w-auto overflow-x-auto pb-1 md:pb-0">
        <span class="text-xs uppercase font-bold text-slate-500 tracking-wider flex items-center gap-1 shrink-0">
          <Filter class="w-3.5 h-3.5 text-church-600" />
          <span>Filtrar:</span>
        </span>
        <div class="flex items-center gap-1.5">
          {#each categories as cat}
            <button
              type="button"
              on:click={() => selectedCategory = cat}
              class="px-3 py-1.5 rounded-xl text-xs font-semibold border transition-all whitespace-nowrap {selectedCategory === cat ? 'bg-church-800 text-white border-church-800 shadow-sm' : 'bg-white text-slate-600 border-slate-200 hover:border-slate-300 hover:bg-slate-50'}"
            >
              {cat}
            </button>
          {/each}
        </div>
      </div>

    </div>

    <!-- Lista de Eventos -->
    {#if loading}
      <div class="text-center py-16">
        <div class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-church-600 border-t-transparent mb-3"></div>
        <p class="text-slate-500 text-sm">Cargando programación...</p>
      </div>
    {:else if filteredEvents.length === 0}
      <div class="text-center py-16 bg-slate-50 rounded-2xl border border-dashed border-slate-300 p-8">
        <Calendar class="w-12 h-12 text-slate-400 mx-auto mb-3" />
        <h4 class="font-serif font-bold text-slate-800 text-lg mb-1">No hay eventos en esta categoría para este mes</h4>
        <p class="text-slate-500 text-sm mb-4">Prueba seleccionando la categoría "Todos" o cambiando de mes.</p>
        <button
          type="button"
          on:click={() => selectedCategory = "Todos"}
          class="px-4 py-2 bg-church-700 text-white rounded-xl text-xs font-semibold hover:bg-church-800 transition-colors"
        >
          Ver todos los eventos
        </button>
      </div>
    {:else}
      <div class="space-y-4">
        {#each filteredEvents as event}
          <div class="bg-white rounded-2xl p-5 sm:p-6 border {event.category.toLowerCase() === 'vigilia' ? 'border-indigo-300 bg-gradient-to-r from-indigo-50/50 via-white to-white ring-2 ring-indigo-200/50' : 'border-slate-200/90'} shadow-sm hover:shadow-md transition-all flex flex-col md:flex-row items-start md:items-center justify-between gap-6 group">
            
            <!-- Columna Fecha y Categoría -->
            <div class="flex items-center gap-4 shrink-0">
              <div class="w-16 h-16 sm:w-20 sm:h-20 rounded-2xl {event.category.toLowerCase() === 'vigilia' ? 'bg-indigo-950 text-indigo-100' : 'bg-church-900 text-white'} flex flex-col items-center justify-center text-center shadow-md group-hover:scale-105 transition-transform">
                <span class="text-[10px] sm:text-xs font-bold tracking-wider text-gold-400 uppercase leading-none">
                  {formatDateInfo(event.date).monthName}
                </span>
                <span class="text-xl sm:text-2xl font-bold font-serif leading-tight">
                  {formatDateInfo(event.date).dayNumber}
                </span>
                <span class="text-[9px] sm:text-[10px] uppercase text-slate-300 font-semibold leading-none">
                  {formatDateInfo(event.date).dayName.slice(0, 3)}
                </span>
              </div>

              <div class="flex flex-col gap-1">
                <div class="flex items-center gap-2">
                  <span class="px-2.5 py-0.5 rounded-full text-xs font-semibold border {getCategoryStyles(event.category)}">
                    {event.category}
                  </span>
                  {#if event.is_featured}
                    <span class="px-2 py-0.5 rounded-full text-[10px] font-bold bg-amber-400 text-amber-950 uppercase tracking-wider">
                      Destacado
                    </span>
                  {/if}
                </div>
                <div class="flex items-center gap-1.5 text-slate-600 text-xs sm:text-sm font-medium">
                  <Clock class="w-3.5 h-3.5 text-church-600" />
                  <span>{event.time}</span>
                </div>
                <div class="flex items-center gap-1.5 text-slate-500 text-xs">
                  <MapPin class="w-3.5 h-3.5 text-slate-400" />
                  <span>{event.location}</span>
                </div>
              </div>
            </div>

            <!-- Columna Título, Descripción y Cita Bíblica -->
            <div class="flex-1 max-w-2xl">
              <h3 class="font-serif font-bold text-lg sm:text-xl text-slate-900 group-hover:text-church-700 transition-colors mb-1.5">
                {event.title}
              </h3>
              <p class="text-slate-600 text-sm leading-relaxed mb-2">
                {event.description}
              </p>
              {#if event.scripture}
                <div class="inline-flex items-center gap-1.5 text-xs text-gold-800 bg-gold-50 border border-gold-200/80 px-2.5 py-1 rounded-lg italic font-serif">
                  <BookOpen class="w-3.5 h-3.5 text-gold-600" />
                  <span>{event.scripture}</span>
                </div>
              {/if}
            </div>

            <!-- Columna Botones de Acción -->
            <div class="flex items-center gap-2 w-full md:w-auto justify-end border-t md:border-t-0 pt-3 md:pt-0 border-slate-100 shrink-0">
              <button
                type="button"
                on:click={() => shareEvent(event)}
                class="flex-1 md:flex-initial inline-flex items-center justify-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-semibold text-emerald-800 bg-emerald-50 hover:bg-emerald-100 border border-emerald-200 transition-colors"
                title="Compartir por WhatsApp"
              >
                <Share2 class="w-3.5 h-3.5" />
                <span>WhatsApp</span>
              </button>

              <button
                type="button"
                on:click={() => addToCalendar(event)}
                class="flex-1 md:flex-initial inline-flex items-center justify-center gap-1.5 px-3.5 py-2 rounded-xl text-xs font-semibold text-church-800 bg-church-50 hover:bg-church-100 border border-church-200 transition-colors"
                title="Añadir a Google Calendar"
              >
                <Calendar class="w-3.5 h-3.5" />
                <span>Calendario</span>
              </button>
            </div>

          </div>
        {/each}
      </div>
    {/if}

    <!-- Pie Informativo de Programación -->
    <div class="mt-12 bg-slate-100/80 rounded-2xl p-6 border border-slate-200 flex flex-col sm:flex-row items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-church-800 text-white flex items-center justify-center shrink-0">
          <Calendar class="w-5 h-5" />
        </div>
        <div>
          <h4 class="font-serif font-bold text-slate-800 text-sm sm:text-base">
            ¿Deseas recibir la programación mensual directamente en tu WhatsApp?
          </h4>
          <p class="text-xs sm:text-sm text-slate-600">
            Únete a nuestra lista de difusión de la Iglesia Cristiana Nueva Vida en Bremen.
          </p>
        </div>
      </div>

      <a
        href="https://wa.me/573113274403?text=Hola%20Pastores,%20por%20favor%20agr%C3%A9guenme%20para%20recibir%20la%20programaci%C3%B3n%20mensual%20de%20la%20iglesia"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-xs sm:text-sm font-semibold text-white bg-emerald-700 hover:bg-emerald-800 shadow-sm transition-colors whitespace-nowrap"
      >
        <span>Recibir Programación en WhatsApp</span>
      </a>
    </div>

  </div>
</section>

<!-- Modal con Instrucciones para Modificar desde Google Sheets -->
{#if showInstructionsModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="bg-white rounded-3xl max-w-lg w-full p-6 sm:p-8 shadow-2xl border border-slate-200 relative animate-in zoom-in-95 duration-200">
      
      <div class="flex items-center justify-between pb-4 border-b border-slate-100 mb-4">
        <div class="flex items-center gap-2.5 text-church-900">
          <div class="w-9 h-9 rounded-xl bg-emerald-100 text-emerald-700 flex items-center justify-center">
            <FileSpreadsheet class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-serif font-bold text-lg text-slate-900">
              Modificar la Programación
            </h3>
            <span class="text-xs text-slate-500">Fácil desde tu celular o PC con Google Sheets</span>
          </div>
        </div>
        <button
          type="button"
          on:click={() => showInstructionsModal = false}
          class="text-slate-400 hover:text-slate-600 text-xl font-bold p-1 leading-none"
        >
          ✕
        </button>
      </div>

      <div class="space-y-4 text-xs sm:text-sm text-slate-600 leading-relaxed">
        <div class="p-3 bg-emerald-50 border border-emerald-200 rounded-xl text-emerald-900">
          <strong>¡Cero código requerido!</strong> Solo abres tu hoja de Google Sheets en tu celular o computador, agregas una fila con la fecha y hora del culto, y la web se actualiza al instante.
        </div>

        <div>
          <h4 class="font-bold text-slate-800 mb-1.5 flex items-center gap-1.5">
            <span class="w-5 h-5 rounded-full bg-church-800 text-white flex items-center justify-center text-xs">1</span>
            <span>Columnas de la Hoja de Google:</span>
          </h4>
          <div class="bg-slate-100 p-2.5 rounded-xl font-mono text-[11px] text-slate-800 overflow-x-auto">
            fecha | hora | titulo | categoria | descripcion | cita_biblica | lugar | destacado
          </div>
        </div>

        <div>
          <h4 class="font-bold text-slate-800 mb-1.5 flex items-center gap-1.5">
            <span class="w-5 h-5 rounded-full bg-church-800 text-white flex items-center justify-center text-xs">2</span>
            <span>Ejemplo de Fila:</span>
          </h4>
          <div class="bg-slate-50 border border-slate-200 p-2.5 rounded-xl text-xs space-y-1">
            <div><strong>fecha:</strong> 2026-11-20</div>
            <div><strong>hora:</strong> 8:00 PM - 1:00 AM</div>
            <div><strong>titulo:</strong> Gran Vigilia de Oración y Clamor</div>
            <div><strong>categoria:</strong> Vigilia</div>
            <div><strong>descripcion:</strong> Noche de clamor unida por las familias.</div>
            <div><strong>cita_biblica:</strong> Jeremías 33:3</div>
            <div><strong>destacado:</strong> SI</div>
          </div>
        </div>

        <div>
          <h4 class="font-bold text-slate-800 mb-1.5 flex items-center gap-1.5">
            <span class="w-5 h-5 rounded-full bg-church-800 text-white flex items-center justify-center text-xs">3</span>
            <span>Publicar la Hoja (Solo se hace 1 vez):</span>
          </h4>
          <p class="text-xs text-slate-600">
            En Google Sheets: ve a <strong>Archivo &gt; Compartir &gt; Publicar en la web</strong> &gt; Elige <strong>Valores separados por comas (.csv)</strong> &gt; Copia el enlace y pégalo en el archivo de configuración <code>src/config.ts</code>.
          </p>
        </div>
      </div>

      <div class="mt-6 pt-4 border-t border-slate-100 flex justify-end">
        <button
          type="button"
          on:click={() => showInstructionsModal = false}
          class="px-5 py-2.5 bg-church-800 text-white rounded-xl text-xs font-semibold hover:bg-church-900 transition-colors"
        >
          Entendido
        </button>
      </div>

    </div>
  </div>
{/if}
