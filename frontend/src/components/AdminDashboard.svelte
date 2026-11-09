<script lang="ts">
  import { onMount } from 'svelte';
  import { logout, exportEventsJson, exportEventsCsv, exportPrayersJson, exportGalleryJson, downloadExport } from '../services/admin';
  import {
    LayoutDashboard,
    Calendar,
    Image,
    LogOut,
    Shield,
    ArrowRight,
    Download,
    Lock,
  } from 'lucide-svelte';
  import EventListAdmin from './EventListAdmin.svelte';
  import EventForm from './EventForm.svelte';
  import GalleryUpload from './GalleryUpload.svelte';
  import GalleryPublic from './GalleryPublic.svelte';
  import EventPreview from './EventPreview.svelte';
  import ChangePasswordModal from './ChangePasswordModal.svelte';
  import type { MonthlyEvent } from '../types';

  let activeSection = 'events'; // 'events' | 'gallery' | 'users'
  let showUploadModal = false;
  let showEventModal = false;
  let showPreviewModal = false;
  let showPasswordModal = false;
  let editingEvent: MonthlyEvent | null = null;
  let previewingEvent: MonthlyEvent | null = null;

  onMount(() => {
    const token = localStorage.getItem('icnvb_admin_token');
    if (!token) {
      window.location.href = '/admin/login';
    }
  });

  function handleLogout() {
    logout();
  }

  async function handleExport(type: 'events-json' | 'events-csv' | 'prayers' | 'gallery') {
    try {
      let result;
      switch (type) {
        case 'events-json':
          result = await exportEventsJson();
          break;
        case 'events-csv':
          result = await exportEventsCsv();
          break;
        case 'prayers':
          result = await exportPrayersJson();
          break;
        case 'gallery':
          result = await exportGalleryJson();
          break;
      }
      downloadExport(result);
    } catch (err) {
      alert(err instanceof Error ? err.message : 'Error exportando datos');
    }
  }
</script>

<div class="min-h-screen bg-slate-50">
  <header class="bg-white border-b border-slate-200 sticky top-0 z-40">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex justify-between h-16 items-center">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-xl bg-church-800 text-white flex items-center justify-center">
            <Shield class="w-5 h-5" />
          </div>
          <h1 class="font-serif text-xl font-bold text-slate-900 tracking-tight">
            Panel de Administración
          </h1>
        </div>

        <div class="flex items-center gap-4">
          <!-- Exportación de datos -->
          <div class="flex items-center gap-2 border-l border-slate-200 pl-4">
            <button
              on:click={() => handleExport('events-json')}
              class="p-2 text-slate-600 hover:text-church-700 hover:bg-slate-50 rounded-lg transition-colors"
              title="Exportar eventos (JSON)"
            >
              <Download class="w-4 h-4" />
            </button>
            <button
              on:click={() => handleExport('events-csv')}
              class="p-2 text-slate-600 hover:text-church-700 hover:bg-slate-50 rounded-lg transition-colors"
              title="Exportar eventos (CSV)"
            >
              <Download class="w-4 h-4" />
            </button>
            <button
              on:click={() => handleExport('prayers')}
              class="p-2 text-slate-600 hover:text-church-700 hover:bg-slate-50 rounded-lg transition-colors"
              title="Exportar peticiones de oración"
            >
              <Download class="w-4 h-4" />
            </button>
            <button
              on:click={() => handleExport('gallery')}
              class="p-2 text-slate-600 hover:text-church-700 hover:bg-slate-50 rounded-lg transition-colors"
              title="Exportar galería"
            >
              <Download class="w-4 h-4" />
            </button>
          </div>

          <!-- Cambio de contraseña -->
          <button
            on:click={() => (showPasswordModal = true)}
            class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-church-700 transition-colors border border-transparent"
          >
            <Lock class="w-4 h-4" />
            <span>Cambiar contraseña</span>
          </button>

          <button
            on:click={handleLogout}
            class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-red-600 transition-colors border border-transparent"
          >
            <LogOut class="w-4 h-4" />
            <span>Cerrar sesión</span>
          </button>
        </div>
      </div>
    </div>
  </header>

  <div class="flex flex-1 overflow-hidden">
    <aside class="w-64 bg-white border-r border-slate-200 hidden lg:block">
      <div class="p-5 space-y-1">
        <nav class="space-y-2">
          <a
            href="#events"
            on:click={() => (activeSection = 'events')}
            class={`flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all ${
              activeSection === 'events'
                ? 'bg-church-50 text-church-800 border-r-2 border-church-800 shadow-sm'
                : 'text-slate-600 hover:bg-slate-50 hover:text-church-700'
            }`}
          >
            <Calendar class="w-4 h-4" />
            <span>Programación</span>
          </a>
          <a
            href="#gallery"
            on:click={() => (activeSection = 'gallery')}
            class={`flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all ${
              activeSection === 'gallery'
                ? 'bg-church-50 text-church-800 border-r-2 border-church-800 shadow-sm'
                : 'text-slate-600 hover:bg-slate-50 hover:text-church-700'
            }`}
          >
            <Image class="w-4 h-4" />
            <span>Galería</span>
          </a>
        </nav>
      </div>
    </aside>

    <main class="flex-1 overflow-y-auto p-6 sm:p-8 lg:p-12">
      <section id="events" class={`space-y-8 ${activeSection !== 'events' ? 'hidden' : 'block'}`}>
        <div class="flex justify-between items-start md:items-center gap-4 mb-6">
          <div>
            <h2 class="font-serif text-2xl font-bold text-slate-900 mb-1">
              Gestión de Programación
            </h2>
            <p class="text-slate-600 text-sm">
              Crea, edita y elimina eventos de la iglesia.
            </p>
          </div>
          <button
            on:click={() => { editingEvent = null; showEventModal = true; }}
            class="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl font-semibold text-white bg-gradient-to-r from-church-700 to-church-900 hover:from-church-800 hover:to-church-950 shadow-md shadow-church-900/20 transition-all transform hover:-translate-y-0.5"
          >
            <ArrowRight class="w-4 h-4" />
            <span>Nuevo Evento</span>
          </button>
        </div>
        <EventListAdmin
          onEdit={(event) => { editingEvent = event; showEventModal = true; }}
          onPreview={(event) => { previewingEvent = event; showPreviewModal = true; }}
        />
      </section>

      <section id="gallery" class={`space-y-8 ${activeSection !== 'gallery' ? 'hidden' : 'block'}`}>
        <div class="flex justify-between items-start md:items-center gap-4 mb-6">
          <div>
            <h2 class="font-serif text-2xl font-bold text-slate-900 mb-1">
              Galería de Fotos y Videos
            </h2>
            <p class="text-slate-600 text-sm">
              Sube y gestiona fotos y videos de las actividades de la iglesia.
            </p>
          </div>
          <button
            on:click={() => (showUploadModal = true)}
            class="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl font-semibold text-white bg-gradient-to-r from-gold-400 to-gold-500 hover:from-gold-300 hover:to-gold-400 shadow-md shadow-gold-500/20 transition-all transform hover:-translate-y-0.5"
          >
            <Image class="w-4 h-4 text-gold-900" />
            <span>Subir Archivo</span>
          </button>
        </div>
        <GalleryPublic adminMode={true} />
      </section>
    </main>
  </div>

  {#if showUploadModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-sm">
      <div class="bg-white rounded-3xl max-w-md w-full p-6 shadow-2xl border border-slate-200">
        <div class="flex items-center justify-between mb-6">
          <h3 class="font-serif text-xl font-bold text-slate-900">Subir a Galería</h3>
          <button
            on:click={() => (showUploadModal = false)}
            class="text-slate-400 hover:text-slate-600 text-xl p-1"
          >
            ✕
          </button>
        </div>
        <GalleryUpload onClose={() => (showUploadModal = false)} />
      </div>
    </div>
  {/if}

  {#if showEventModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-sm">
      <div class="bg-white rounded-3xl max-w-2xl w-full p-6 shadow-2xl border border-slate-200">
        <div class="flex items-center justify-between mb-6">
          <h3 class="font-serif text-xl font-bold text-slate-900">
            {editingEvent ? 'Editar Evento' : 'Nuevo Evento'}
          </h3>
          <button
            on:click={() => (showEventModal = false)}
            class="text-slate-400 hover:text-slate-600 text-xl p-1"
          >
            ✕
          </button>
        </div>
        <EventForm {editingEvent} onClose={() => (showEventModal = false)} />
      </div>
    </div>
  {/if}

  {#if showPreviewModal}
    <EventPreview
      {previewingEvent}
      onClose={() => (showPreviewModal = false)}
      onEdit={(event) => {
        editingEvent = event;
        showPreviewModal = false;
        showEventModal = true;
      }}
      onPublish={() => {
        showPreviewModal = false;
        // Aquí se podría agregar lógica para marcar como publicado
        alert('Evento publicado');
      }}
    />
  {/if}

  {#if showPasswordModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-sm">
      <div class="bg-white rounded-3xl max-w-md w-full p-6 shadow-2xl border border-slate-200">
        <div class="flex items-center justify-between mb-6">
          <h3 class="font-serif text-xl font-bold text-slate-900">Cambiar Contraseña</h3>
          <button
            on:click={() => (showPasswordModal = false)}
            class="text-slate-400 hover:text-slate-600 text-xl p-1"
          >
            ✕
          </button>
        </div>
        <ChangePasswordModal onClose={() => (showPasswordModal = false)} />
      </div>
    </div>
  {/if}
</div>