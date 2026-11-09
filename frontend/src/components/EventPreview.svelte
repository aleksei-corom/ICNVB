<script lang="ts">
  import type { MonthlyEvent } from '../types';
  import { Calendar, Clock, MapPin, User, Music, BookOpen, X, Check, Edit3 } from 'lucide-svelte';

  export let event: MonthlyEvent | null = null;
  export let onClose: () => void = () => {};
  export let onEdit: (event: MonthlyEvent) => void = () => {};
  export let onPublish: () => void = () => {};

  function formatDate(dateStr: string) {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('es-CO', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' });
  }

  function getCategoryColor(category: string) {
    const colors: Record<string, string> = {
      'Culto General': 'bg-church-100 text-church-800 border-church-200',
      'Jóvenes': 'bg-amber-100 text-amber-800 border-amber-200',
      'Damas': 'bg-pink-100 text-pink-800 border-pink-200',
      'Ayuno': 'bg-emerald-100 text-emerald-800 border-emerald-200',
      'Vigilia': 'bg-indigo-900 text-indigo-100 border-indigo-700 font-bold',
      'Enseñanza': 'bg-sky-100 text-sky-800 border-sky-200',
      'Grupos Familiares': 'bg-purple-100 text-purple-800 border-purple-200',
    };
    return colors[category] || 'bg-slate-100 text-slate-800 border-slate-200';
  }
</script>

{#if event}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-900/60 backdrop-blur-sm">
    <div class="bg-white rounded-3xl max-w-2xl w-full p-6 shadow-2xl border border-slate-200">
      <div class="flex items-center justify-between mb-6">
        <h3 class="font-serif text-xl font-bold text-slate-900">
          Vista Previa del Evento
        </h3>
        <button
          on:click={onClose}
          class="text-slate-400 hover:text-slate-600 text-xl p-1"
        >
          ✕
        </button>
      </div>

      <div class="space-y-6">
        <!-- Encabezado -->
        <div class="flex justify-between items-start">
          <div>
            <span class={`inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[10px] font-bold ${getCategoryColor(event.category)} uppercase`}>
              {event.category}
            </span>
            {#if event.is_featured}
              <span class="ml-2 inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[10px] font-bold bg-amber-100 text-amber-950 uppercase">
                <span class="w-2 h-2 rounded-full bg-amber-500 animate-ping"></span>
                Destacado
              </span>
            {/if}
          </div>
        </div>

        <h2 class="font-serif text-2xl font-bold text-slate-900 mb-2">
          {event.title}
        </h2>

        <!-- Fecha, hora, lugar -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs text-slate-600">
          <div class="flex items-center gap-2">
            <Calendar class="w-4 h-4" />
            <span><strong>Fecha:</strong> {formatDate(event.date)}</span>
          </div>
          <div class="flex items-center gap-2">
            <Clock class="w-4 h-4" />
            <span><strong>Hora:</strong> {event.time}</span>
          </div>
          <div class="flex items-center gap-2">
            <MapPin class="w-4 h-4" />
            <span><strong>Lugar:</strong> {event.location}</span>
          </div>
        </div>

        <!-- Predicador y alabanza -->
        {#if event.predicador || event.director_alabanza}
          <div class="flex items-center gap-4">
            {#if event.predicador}
              <div class="flex items-center gap-2 bg-church-50 text-church-700 px-3 py-2 rounded-xl border border-church-100">
                <User class="w-4 h-4" />
                <span><strong>Predicador:</strong> {event.predicador}</span>
              </div>
            {/if}
            {#if event.director_alabanza}
              <div class="flex items-center gap-2 bg-gold-50 text-gold-800 px-3 py-2 rounded-xl border border-gold-100">
                <Music class="w-4 h-4" />
                <span><strong>Alabanza:</strong> {event.director_alabanza}</span>
              </div>
            {/if}
          </div>
        {/if}

        <!-- Cita bíblica -->
        {#if event.scripture}
          <div class="bg-gold-50 border border-gold-200/80 px-4 py-3 rounded-xl italic font-serif text-gold-800">
            <BookOpen class="w-4 h-4 inline mr-2 text-gold-600" />
            "{event.scripture}"
          </div>
        {/if}

        <!-- Descripción -->
        <div class="pt-4 border-t border-slate-100">
          <h4 class="text-sm font-semibold text-slate-900 uppercase tracking-wider mb-2">
            Descripción
          </h4>
          <p class="text-slate-600 text-sm leading-relaxed">
            {event.description}
          </p>
        </div>
      </div>

      <!-- Botones de acción -->
      <div class="flex justify-end gap-3 pt-6 border-t border-slate-100">
        <button
          on:click={() => {
            onEdit(event);
            onClose();
          }}
          class="px-6 py-2.5 rounded-xl font-semibold text-white bg-church-800 hover:bg-church-900 transition-all"
        >
          <Edit3 class="w-4 h-4 mr-2" />
          Editar Evento
        </button>
        <button
          on:click={() => {
            onPublish();
            onClose();
          }}
          class="px-6 py-2.5 rounded-xl font-semibold text-church-700 bg-white border border-slate-200 hover:bg-slate-50 transition-all"
        >
          <Check class="w-4 h-4 mr-2" />
          Publicar
        </button>
        <button
          on:click={onClose}
          class="px-4 py-2.5 rounded-xl text-slate-600 bg-white border border-slate-200 hover:bg-slate-50 transition-all"
        >
          <X class="w-4 h-4" />
          Cerrar
        </button>
      </div>
    </div>
  </div>
{/if}