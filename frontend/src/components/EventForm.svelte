<script lang="ts">
  import { createEvent, updateEvent } from '../services/admin';
  import type { MonthlyEvent } from '../types';
  import { APP_CONFIG } from '../config';

  export let editingEvent: MonthlyEvent | null = null;
  export let onClose: () => void = () => {};

  let title = editingEvent?.title || '';
  let date = editingEvent?.date || '';
  let time = editingEvent?.time || '';
  let category = editingEvent?.category || 'Culto General';
  let description = editingEvent?.description || '';
  let scripture = editingEvent?.scripture || '';
  let predicador = editingEvent?.predicador || '';
  let directorAlabanza = editingEvent?.director_alabanza || '';
  let location = editingEvent?.location || '';
  let isFeatured = editingEvent?.is_featured || false;
  let isSubmitting = false;
  let error = '';

  const categories = APP_CONFIG.eventCategories;

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    isSubmitting = true;

    const input = {
      title: title.trim(),
      date: date.trim(),
      time: time.trim(),
      category: category.trim(),
      description: description.trim(),
      scripture: scripture.trim() || undefined,
      predicador: predicador.trim() || undefined,
      director_alabanza: directorAlabanza.trim() || undefined,
      is_featured: isFeatured,
      location: location.trim(),
    };

    try {
      if (editingEvent) {
        await updateEvent(editingEvent.id, input);
      } else {
        await createEvent(input);
      }
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Error guardando el evento';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-5">
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="event-title" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Título del Evento *
      </label>
      <input
        id="event-title"
        type="text"
        bind:value={title}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Culto Dominical"
      />
    </div>

    <div>
      <label for="event-category" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Categoría *
      </label>
      <select
        id="event-category"
        bind:value={category}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      >
        {#each categories as cat}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
    </div>

    <div>
      <label for="event-date" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Fecha *
      </label>
      <input
        id="event-date"
        type="date"
        bind:value={date}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      />
    </div>

    <div>
      <label for="event-time" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Hora *
      </label>
      <input
        id="event-time"
        type="time"
        bind:value={time}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      />
    </div>
  </div>

  <div>
    <label for="event-description" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Descripción *
    </label>
    <textarea
      id="event-description"
      bind:value={description}
      rows="3"
      required
      class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      placeholder="Describe el evento..."
    ></textarea>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    <div>
      <label for="event-predicador" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Predicador
      </label>
      <input
        id="event-predicador"
        type="text"
        bind:value={predicador}
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Pastor Benjamín Ramos"
      />
    </div>

    <div>
      <label for="event-director" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Director de Alabanza
      </label>
      <input
        id="event-director"
        type="text"
        bind:value={directorAlabanza}
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Hermano Carlos"
      />
    </div>

    <div>
      <label for="event-location" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Lugar
      </label>
      <input
        id="event-location"
        type="text"
        bind:value={location}
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Templo Central"
      />
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="event-scripture" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Cita Bíblica
      </label>
      <input
        id="event-scripture"
        type="text"
        bind:value={scripture}
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Juan 3:16"
      />
    </div>

    <div class="flex items-center">
      <input
        id="event-featured"
        type="checkbox"
        bind:checked={isFeatured}
        class="w-4 h-4 rounded text-church-500 focus:ring-church-500 bg-white border-slate-200"
      />
      <label for="event-featured" class="ml-2 text-xs font-semibold text-slate-600 uppercase tracking-wider">
        Evento Destacado
      </label>
    </div>
  </div>

  {#if error}
    <div class="p-3 rounded-xl bg-red-50 border border-red-200 text-red-700 text-sm">
      {error}
    </div>
  {/if}

  <div class="flex justify-end gap-3 pt-4 border-t border-slate-100">
    <button
      type="button"
      on:click={onClose}
      class="px-4 py-2 rounded-xl text-sm font-medium text-slate-600 bg-white border border-slate-200 hover:bg-slate-50 transition-colors"
    >
      Cancelar
    </button>
    <button
      type="submit"
      disabled={isSubmitting}
      class="px-6 py-2 rounded-xl font-semibold text-white bg-church-800 hover:bg-church-900 disabled:opacity-50 transition-all"
    >
      {#if isSubmitting}
        Guardando...
      {:else}
        {editingEvent ? 'Actualizar Evento' : 'Crear Evento'}
      {/if}
    </button>
  </div>
</form>