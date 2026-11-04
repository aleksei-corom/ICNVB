<script lang="ts">
  import { Calendar, HeartHandshake, MapPin, Sparkles, Clock, ArrowRight, ShieldCheck } from 'lucide-svelte';

  export let churchName = "Iglesia Cristiana Nueva Vida";
  export let slogan = "Un lugar para creer, pertenecer y crecer en Cristo";
</script>

<section id="inicio" class="relative overflow-hidden bg-gradient-to-b from-church-950 via-church-900 to-slate-900 text-white pt-12 pb-24 md:pt-20 md:pb-32">
  <!-- Efectos decorativos de fondo -->
  <div class="absolute inset-0 opacity-15 pointer-events-none">
    <div class="absolute -top-40 -right-40 w-96 h-96 rounded-full bg-gold-400 blur-3xl"></div>
    <div class="absolute -bottom-20 -left-20 w-96 h-96 rounded-full bg-church-500 blur-3xl"></div>
  </div>

  <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="text-center max-w-4xl mx-auto">
      
      <!-- Badge de Ubicación y Bienvenida -->
      <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white/10 backdrop-blur-md border border-white/15 text-gold-300 text-xs sm:text-sm font-semibold mb-6 shadow-inner animate-pulse">
        <Sparkles class="w-4 h-4 text-gold-400" />
        <span>Bienvenidos a casa • Corregimiento Bremen, Sincelejo, Sucre</span>
      </div>

      <!-- Título Principal -->
      <h1 class="font-serif text-3xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-white mb-6 leading-tight">
        {churchName}
      </h1>

      <p class="text-xl sm:text-2xl text-gold-200/90 font-serif italic mb-6">
        "{slogan}"
      </p>

      <p class="text-base sm:text-lg text-slate-300 max-w-2xl mx-auto mb-10 leading-relaxed">
        Somos una familia de fe fundamentada en la Santa Palabra de Dios. Un espacio de adoración, comunión fraternal, restauración espiritual y bendición para cada hogar en Bremen y Sincelejo.
      </p>

      <!-- Botones de Acción -->
      <div class="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16">
        <a 
          href="#programacion" 
          class="w-full sm:w-auto inline-flex items-center justify-center gap-2.5 px-7 py-3.5 rounded-xl font-semibold text-base text-church-950 bg-gradient-to-r from-gold-400 to-gold-500 hover:from-gold-300 hover:to-gold-400 shadow-lg shadow-gold-500/20 hover:shadow-xl transition-all transform hover:-translate-y-0.5"
        >
          <Calendar class="w-5 h-5 text-church-950" />
          <span>Ver Programación del Mes</span>
        </a>

        <a 
          href="#oracion" 
          class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-7 py-3.5 rounded-xl font-semibold text-base text-white bg-white/10 hover:bg-white/20 border border-white/20 backdrop-blur-md transition-all hover:-translate-y-0.5"
        >
          <HeartHandshake class="w-5 h-5 text-gold-400" />
          <span>Solicitar Oración</span>
        </a>

        <a 
          href="#ubicacion" 
          class="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-7 py-3.5 rounded-xl font-semibold text-base text-church-200 hover:text-white transition-colors"
        >
          <span>Cómo llegar</span>
          <ArrowRight class="w-4 h-4" />
        </a>
      </div>

      <!-- Tarjeta Flotante con Horarios Resumidos -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3 sm:gap-4 text-left">
        
        <div class="p-4 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-gold-400 mb-1">
            <Clock class="w-4 h-4" />
            <span class="text-xs uppercase font-bold tracking-wider">Domingos</span>
          </div>
          <div class="font-bold text-white text-base">9:00 AM - 11:30 AM</div>
          <div class="text-xs text-slate-300 mt-1">Culto General & Escuela Bíblica Infantil</div>
        </div>

        <div class="p-4 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-gold-400 mb-1">
            <Clock class="w-4 h-4" />
            <span class="text-xs uppercase font-bold tracking-wider">Miércoles</span>
          </div>
          <div class="font-bold text-white text-base">7:00 PM - 9:00 PM</div>
          <div class="text-xs text-slate-300 mt-1">Oración & Doctrina Bíblica</div>
        </div>

        <div class="p-4 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-gold-400 mb-1">
            <Clock class="w-4 h-4" />
            <span class="text-xs uppercase font-bold tracking-wider">Jueves</span>
          </div>
          <div class="font-bold text-white text-base">9 AM Ayuno / 7 PM Damas</div>
          <div class="text-xs text-slate-300 mt-1">Intercesión y Mujeres de Fe</div>
        </div>

        <div class="p-4 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-gold-400 mb-1">
            <Clock class="w-4 h-4" />
            <span class="text-xs uppercase font-bold tracking-wider">Viernes</span>
          </div>
          <div class="font-bold text-white text-base">7:00 PM - 9:00 PM</div>
          <div class="text-xs text-slate-300 mt-1">Jóvenes y Jovencitos</div>
        </div>

      </div>

    </div>
  </div>

  <!-- Versículo de Bienvenida inferior -->
  <div class="max-w-4xl mx-auto px-4 mt-12 text-center">
    <blockquote class="text-sm sm:text-base text-church-200/80 italic font-serif">
      "Yo me alegré con los que me decían: A la casa de Jehová iremos." — Salmos 122:1
    </blockquote>
  </div>
</section>

