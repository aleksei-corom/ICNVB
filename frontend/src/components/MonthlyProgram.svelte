<script lang="ts">
  import { onMount } from 'svelte';
  import { Calendar, Filter, Share2, Sparkles, MapPin, Clock, BookOpen, User, Music, RefreshCw, FileSpreadsheet, ExternalLink, Info } from 'lucide-svelte';
  import type { MonthlyEvent } from '../types';
  import { APP_CONFIG } from '../config';
  import { fetchEventsFromGoogleSheets } from '../services/sheets';
  import {
    generateMonths,
    CATEGORIES,
    DEFAULT_EVENTS,
    formatDateInfo,
    getCategoryStyles,
    filterEvents,
  } from '../services/program';
  import ProgramInstructionsModal from './ProgramInstructionsModal.svelte';

  let events: MonthlyEvent[] = [];
  let loading = true;
  let isRefreshing = false;
  let isGoogleSheetsActive = Boolean(APP_CONFIG.googleSheetsCsvUrl && APP_CONFIG.googleSheetsCsvUrl.trim().length > 0);
  let showInstructionsModal = false;

  let selectedCategory = 'Todos';
  let selectedMonth = generateMonths()[0]?.value ?? new Date().toISOString().slice(0, 7);

  const months = generateMonths();

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
      const scheduleUrl = APP_CONFIG.apiBaseUrl 
        ? `${APP_CONFIG.apiBaseUrl}/api/schedule?month=${selectedMonth}` 
        : `/api/schedule?month=${selectedMonth}`;
      const res = await fetch(scheduleUrl);
      if (res.ok) {
        const json = await res.json();
        if (json.success && json.data && json.data.length > 0) {
          events = json.data;
          return;
        }
      }

      // 3. Fallback a eventos predeterminados
      events = DEFAULT_EVENTS;
    } catch {
      events = DEFAULT_EVENTS;
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
  $: filteredEvents = filterEvents(events, selectedMonth, selectedCategory);

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
          on:click={() => (showInstructionsModal = true)}
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
          {#each CATEGORIES as cat}
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
          on:click={() => selectedCategory = 'Todos'}
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
                <div class="inline-flex items-center gap-1.5 text-xs text-gold-800 bg-gold-50 border border-gold-200/80 px-2.5 py-1 rounded-lg italic font-serif mb-2">
                  <BookOpen class="w-3.5 h-3.5 text-gold-600" />
                  <span>{event.scripture}</span>
                </div>
              {/if}
              {#if event.predicador || event.directorAlabanza}
                <div class="flex flex-col sm:flex-row gap-2 sm:gap-3 text-xs sm:text-sm">
                  {#if event.predicador}
                    <div class="inline-flex items-center gap-1 bg-church-50 text-church-700 px-2.5 py-1 rounded-lg border border-church-100">
                      <User class="w-3 h-3" />
                      <span class="font-semibold">Predicador: {event.predicador}</span>
                    </div>
                  {/if}
                  {#if event.directorAlabanza}
                    <div class="inline-flex items-center gap-1 bg-gold-50 text-gold-800 px-2.5 py-1 rounded-lg border border-gold-100">
                      <Music class="w-3 h-3" />
                      <span class="font-semibold">Alabanza: {event.directorAlabanza}</span>
                    </div>
                  {/if}
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

<ProgramInstructionsModal show={showInstructionsModal} onClose={() => showInstructionsModal = false} />
