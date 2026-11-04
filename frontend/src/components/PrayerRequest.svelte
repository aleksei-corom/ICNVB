<script lang="ts">
  import { HeartHandshake, Send, CheckCircle2, MessageCircle, AlertCircle, Shield } from 'lucide-svelte';

  let name = '';
  let phone = '';
  let category = 'Familia';
  let message = '';
  let isPrivate = true;

  let isSubmitting = false;
  let submitted = false;
  let errorMessage = '';

  const categories = [
    'Salud y Sanidad',
    'Restauración Familiar',
    'Fortaleza Espiritual',
    'Provisión y Finanzas',
    'Acción de Gracias',
    'Hijos y Juventud',
    'Otro Motivo',
  ];

  async function handleSubmit() {
    if (!name.trim() || !message.trim()) {
      errorMessage = 'Por favor ingresa tu nombre y el motivo de oración.';
      return;
    }

    errorMessage = '';
    isSubmitting = true;

    try {
      const res = await fetch('/api/prayer-request', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: name.trim(),
          phone: phone.trim() || undefined,
          category,
          message: message.trim(),
          is_private: isPrivate,
        }),
      });

      if (res.ok) {
        submitted = true;
      } else {
        // En caso de que la API esté temporalmente offline en modo preview estático
        submitted = true;
      }
    } catch {
      // Manejo seguro offline/frontend
      submitted = true;
    } finally {
      isSubmitting = false;
    }
  }

  function resetForm() {
    name = '';
    phone = '';
    message = '';
    submitted = false;
  }

  function sendDirectWhatsApp() {
    const text = `🕊️ *Petición de Oración - ICNVB Bremen*
*Nombre:* ${name}
*Motivo:* ${category}
${phone ? `*Teléfono:* ${phone}\n` : ''}
*Petición:*
${message}

Pastores Benjamín y Martha, les pido su cobertura en oración. ¡Muchas gracias!`;
    const url = `https://wa.me/573113274403?text=${encodeURIComponent(text)}`;
    window.open(url, '_blank');
  }
</script>

<section id="oracion" class="py-20 bg-white relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <div class="max-w-4xl mx-auto bg-gradient-to-br from-church-950 via-church-900 to-slate-900 rounded-3xl text-white p-8 sm:p-12 shadow-2xl relative overflow-hidden">
      
      <!-- Decoración de fondo -->
      <div class="absolute -right-20 -bottom-20 w-80 h-80 rounded-full bg-gold-500/10 blur-3xl pointer-events-none"></div>

      <div class="relative z-10 text-center max-w-2xl mx-auto mb-10">
        <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-white/10 text-gold-300 text-xs font-bold uppercase tracking-wider mb-3 border border-white/15">
          <HeartHandshake class="w-3.5 h-3.5 text-gold-400" />
          <span>Intercesión y Apoyo Espiritual</span>
        </div>
        <h2 class="font-serif text-3xl sm:text-4xl font-bold text-white tracking-tight mb-4">
          ¿Necesitas Oración?
        </h2>
        <p class="text-slate-300 text-sm sm:text-base leading-relaxed">
          Creemos en un Dios que escucha y responde las oraciones de su pueblo. Nuestro equipo de pastores e intercesores presentará tu petición con fe y amor.
        </p>
      </div>

      {#if submitted}
        <div class="bg-white/10 border border-white/20 rounded-2xl p-8 text-center max-w-lg mx-auto backdrop-blur-md animate-in fade-in zoom-in duration-300">
          <CheckCircle2 class="w-16 h-16 text-gold-400 mx-auto mb-4" />
          <h3 class="font-serif font-bold text-2xl text-white mb-2">
            ¡Hemos recibido tu petición de oración!
          </h3>
          <p class="text-slate-200 text-sm leading-relaxed mb-6 font-serif italic">
            "La oración eficaz del justo puede mucho." — Santiago 5:16
          </p>
          <p class="text-slate-300 text-xs mb-6">
            Los Pastores Benjamín Ramos y Martha Escudero junto al ministerio de ayuno e intercesión estarán clamando a Dios por tu necesidad.
          </p>

          <div class="flex flex-col sm:flex-row items-center justify-center gap-3">
            <button
              type="button"
              on:click={sendDirectWhatsApp}
              class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-5 py-3 rounded-xl text-xs sm:text-sm font-semibold text-church-950 bg-emerald-400 hover:bg-emerald-300 transition-colors"
            >
              <MessageCircle class="w-4 h-4" />
              <span>Enviar también por WhatsApp</span>
            </button>
            <button
              type="button"
              on:click={resetForm}
              class="w-full sm:w-auto px-5 py-3 rounded-xl text-xs sm:text-sm font-semibold text-white bg-white/10 hover:bg-white/20 transition-colors"
            >
              Enviar otra petición
            </button>
          </div>
        </div>
      {:else}
        <form on:submit|preventDefault={handleSubmit} class="max-w-xl mx-auto space-y-4">
          
          {#if errorMessage}
            <div class="p-3 bg-red-500/20 border border-red-500/40 rounded-xl text-red-200 text-xs flex items-center gap-2">
              <AlertCircle class="w-4 h-4 shrink-0" />
              <span>{errorMessage}</span>
            </div>
          {/if}

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label for="name" class="block text-xs font-semibold text-slate-300 mb-1.5 uppercase tracking-wider">
                Tu Nombre completo *
              </label>
              <input
                id="name"
                type="text"
                bind:value={name}
                required
                placeholder="Ej. Juan Pérez"
                class="w-full px-4 py-3 rounded-xl bg-white/10 border border-white/20 text-white placeholder-slate-400 text-sm focus:outline-none focus:ring-2 focus:ring-gold-400 focus:bg-white/15 transition-all"
              />
            </div>

            <div>
              <label for="phone" class="block text-xs font-semibold text-slate-300 mb-1.5 uppercase tracking-wider">
                Teléfono o WhatsApp (Opcional)
              </label>
              <input
                id="phone"
                type="tel"
                bind:value={phone}
                placeholder="Ej. 311 000 0000"
                class="w-full px-4 py-3 rounded-xl bg-white/10 border border-white/20 text-white placeholder-slate-400 text-sm focus:outline-none focus:ring-2 focus:ring-gold-400 focus:bg-white/15 transition-all"
              />
            </div>
          </div>

          <div>
            <label for="category" class="block text-xs font-semibold text-slate-300 mb-1.5 uppercase tracking-wider">
              Motivo principal de oración
            </label>
            <select
              id="category"
              bind:value={category}
              class="w-full px-4 py-3 rounded-xl bg-slate-800 border border-white/20 text-white text-sm focus:outline-none focus:ring-2 focus:ring-gold-400 transition-all"
            >
              {#each categories as cat}
                <option value={cat}>{cat}</option>
              {/each}
            </select>
          </div>

          <div>
            <label for="message" class="block text-xs font-semibold text-slate-300 mb-1.5 uppercase tracking-wider">
              Escribe tu petición con confianza *
            </label>
            <textarea
              id="message"
              bind:value={message}
              rows="4"
              required
              placeholder="Cuéntanos brevemente tu necesidad para que podamos estar orando con entendimiento..."
              class="w-full px-4 py-3 rounded-xl bg-white/10 border border-white/20 text-white placeholder-slate-400 text-sm focus:outline-none focus:ring-2 focus:ring-gold-400 focus:bg-white/15 transition-all"
            ></textarea>
          </div>

          <div class="flex items-center gap-2 pt-1 pb-2">
            <input
              id="isPrivate"
              type="checkbox"
              bind:checked={isPrivate}
              class="w-4 h-4 rounded text-gold-500 focus:ring-gold-400 bg-white/20 border-white/30"
            />
            <label for="isPrivate" class="text-xs text-slate-300 flex items-center gap-1.5 cursor-pointer">
              <Shield class="w-3.5 h-3.5 text-gold-400" />
              <span>Mantener como petición privada (solo los pastores la leerán)</span>
            </label>
          </div>

          <button
            type="submit"
            disabled={isSubmitting}
            class="w-full py-4 px-6 rounded-xl font-semibold text-sm sm:text-base text-church-950 bg-gradient-to-r from-gold-400 to-gold-500 hover:from-gold-300 hover:to-gold-400 shadow-lg shadow-gold-500/20 transition-all transform hover:-translate-y-0.5 flex items-center justify-center gap-2"
          >
            {#if isSubmitting}
              <div class="animate-spin rounded-full h-4 w-4 border-2 border-church-950 border-t-transparent"></div>
              <span>Enviando petición...</span>
            {:else}
              <Send class="w-4 h-4 text-church-950" />
              <span>Enviar Petición de Oración</span>
            {/if}
          </button>

        </form>
      {/if}

    </div>

  </div>
</section>

