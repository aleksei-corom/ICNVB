<script lang="ts">
  import { FileSpreadsheet } from 'lucide-svelte';

  export let show = false;
  export let onClose: () => void = () => {};
</script>

{#if show}
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
          on:click={() => onClose()}
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
          on:click={() => onClose()}
          class="px-5 py-2.5 bg-church-800 text-white rounded-xl text-xs font-semibold hover:bg-church-900 transition-colors"
        >
          Entendido
        </button>
      </div>

    </div>
  </div>
{/if}
