<script lang="ts">
  import { onMount } from 'svelte';
  import { getAdminEvents, deleteEvent } from '../services/admin';
  import type { MonthlyEvent } from '../types';
  import { Trash2, Edit3, Calendar, MapPin, User, Music } from 'lucide-svelte';

  export let onEdit: (event: MonthlyEvent) => void = () => {};

  let events: MonthlyEvent[] = [];
  let loading = true;
  let error = '';

  onMount(async () => {
    try {
      events = await getAdminEvents();
    } catch (err) {
      error = 'Error cargando los eventos. Intenta iniciar sesión nuevamente.';
      console.error(err);
    } finally {
      loading = false;
    }
  });

  async function handleDelete(id: string, title: string) {
    if (!confirm(`¿Estás seguro de eliminar el evento "${title}"?`)) return;
    try {
      await deleteEvent(id);
      events = events.filter(e => e.id !== id);
    } catch {
      alert('No se pudo eliminar el evento');
    }
  }

  function formatDate(dateStr: string) {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('es-CO', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' });
  }
</script>

{#if loading}
  <div class="flex items-center justify-center py-20">
    <div class="animate-spin rounded-full h-8 w-8 border-4 border-church-600 border-t-transparent"></div>
  </div>
{:else if error}
  <div class="bg-red-50 border border-red-200 rounded-xl p-6 text-red-700 text-sm">
    {error}
  </div>
{:else}
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    {#if events.length === 0}
      <div class="col-span-3 text-center py-20">
        <Calendar class="w-12 h-12 text-slate-400 mx-auto mb-4" />
        <h3 class="font-serif text-lg font-medium text-slate-900 mb-2">No hay eventos</h3>
        <p class="text-slate-500 text-sm">Crea tu primer evento haciendo clic en "Nuevo Evento"</p>
      </div>
    {/if}
    {#each events as event}
      <div class="bg-white rounded-2xl border border-slate-200 shadow-sm hover:shadow-md transition-all p-6">
        <div class="flex justify-between items-start mb-4">
          <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-[10px] font-bold bg-church-100 text-church-700 uppercase">
            {event.category}
          </span>
          <div class="flex items-center gap-2">
            <button
              on:click={() => onEdit(event)}
              class="p-2 text-slate-400 hover:text-church-700 hover:bg-slate-50 rounded-lg transition-colors"
              title="Editar"
            >
              <Edit3 class="w-4 h-4" />
            </button>
            <button
              on:click={() => handleDelete(event.id, event.title)}
              class="p-2 text-slate-400 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
              title="Eliminar"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>

        <h3 class="font-serif text-lg font-bold text-slate-900 mb-1">{event.title}</h3>
        <p class="text-slate-600 text-sm leading-relaxed mb-4 line-clamp-2">{event.description}</p>

        <div class="space-y-2 text-xs text-slate-500 mb-4">
          <div class="flex items-center gap-2">
            <Calendar class="w-3 h-3" />
            <span>{formatDate(event.date)} • {event.time}</span>
          </div>
          {#if event.location}
            <div class="flex items-center gap-2">
              <MapPin class="w-3 h-3" />
              <span>{event.location}</span>
            </div>
          {/if}
          {#if event.predicador}
            <div class="flex items-center gap-2">
              <User class="w-3 h-3" />
              <span>Predicador: {event.predicador}</span>
            </div>
          {/if}
          {#if event.director_alabanza}
            <div class="flex items-center gap-2">
              <Music class="w-3 h-3" />
              <span>Alabanza: {event.director_alabanza}</span>
            </div>
          {/if}
        </div>

        <div class="flex items-center justify-between pt-4 border-t border-slate-100">
          <span class={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-[10px] font-bold ${
            event.is_featured ? 'bg-amber-100 text-amber-800' : 'bg-slate-100 text-slate-600'
          }`}>
            {event.is_featured ? 'Destacado' : 'Normal'}
          </span>
        </div>
      </div>
    {/each}
  </div>
{/if}