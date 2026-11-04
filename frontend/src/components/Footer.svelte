<script lang="ts">
  import { Heart, MapPin, Phone, MessageCircle, ArrowUp } from 'lucide-svelte';

  function scrollToTop() {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }
</script>

<footer class="bg-gradient-to-b from-church-950 to-slate-950 text-white pt-16 pb-12 border-t border-church-900">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 mb-12">
      
      <!-- Columna 1: Identidad -->
      <div class="space-y-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-church-700 to-church-900 flex items-center justify-center text-gold-400 font-serif font-bold text-lg shadow-md border border-church-700">
            ✝
          </div>
          <span class="font-serif font-bold text-xl text-white">
            ICNVB
          </span>
        </div>
        <p class="font-serif text-base text-gold-200/90 font-semibold">
          Iglesia Cristiana Nueva Vida
        </p>
        <p class="text-xs text-slate-300 leading-relaxed">
          "Un lugar para creer, pertenecer y crecer en Cristo". Predicando el Evangelio de la Gracia y restaurando familias en el corregimiento de Bremen, Sincelejo, Sucre.
        </p>
        <div class="pt-2 text-xs text-slate-400 flex items-center gap-1.5">
          <MapPin class="w-3.5 h-3.5 text-gold-400" />
          <span>Bremen, Sincelejo, Sucre, Colombia</span>
        </div>
      </div>

      <!-- Columna 2: Enlaces Rápidos -->
      <div>
        <h4 class="font-serif font-bold text-sm text-gold-400 uppercase tracking-wider mb-4">
          Navegación
        </h4>
        <ul class="space-y-2 text-xs sm:text-sm text-slate-300">
          <li><a href="#inicio" class="hover:text-gold-300 transition-colors">Inicio</a></li>
          <li><a href="#horarios" class="hover:text-gold-300 transition-colors">Horarios de Cultos</a></li>
          <li><a href="#programacion" class="hover:text-gold-300 transition-colors">Programación Mensual</a></li>
          <li><a href="#pastores" class="hover:text-gold-300 transition-colors">Nuestros Pastores</a></li>
          <li><a href="#ministerios" class="hover:text-gold-300 transition-colors">Ministerios de la Iglesia</a></li>
          <li><a href="#creemos" class="hover:text-gold-300 transition-colors">Declaración de Fe</a></li>
          <li><a href="#oracion" class="hover:text-gold-300 transition-colors">Peticiones de Oración</a></li>
          <li><a href="#ubicacion" class="hover:text-gold-300 transition-colors">Ubicación y Contacto</a></li>
        </ul>
      </div>

      <!-- Columna 3: Horarios de Servicios -->
      <div>
        <h4 class="font-serif font-bold text-sm text-gold-400 uppercase tracking-wider mb-4">
          Reuniones Semanales
        </h4>
        <ul class="space-y-2.5 text-xs text-slate-300">
          <li>
            <strong class="text-white block">Domingos: 9:00 AM - 11:30 AM</strong>
            Culto General y Escuela Dominical
          </li>
          <li>
            <strong class="text-white block">Miércoles: 7:00 PM - 9:00 PM</strong>
            Culto de Oración y Doctrina Bíblica
          </li>
          <li>
            <strong class="text-white block">Jueves: 9:00 AM - 12:00 M</strong>
            Ayuno Congregacional de Intercesión
          </li>
          <li>
            <strong class="text-white block">Jueves: 7:00 PM - 9:00 PM</strong>
            Culto de Damas (Mujeres de Fe)
          </li>
          <li>
            <strong class="text-white block">Viernes: 7:00 PM - 9:00 PM</strong>
            Culto de Jóvenes y Jovencitos
          </li>
          <li>
            <strong class="text-gold-300 block">Vigilias: Programación Mensual</strong>
          </li>
        </ul>
      </div>

      <!-- Columna 4: Contacto Pastoral -->
      <div>
        <h4 class="font-serif font-bold text-sm text-gold-400 uppercase tracking-wider mb-4">
          Líneas Pastorales
        </h4>
        <div class="space-y-3 text-xs">
          <div class="bg-white/5 p-3 rounded-xl border border-white/10">
            <span class="text-slate-400 block text-[11px]">Pastor Benjamín Ramos</span>
            <a 
              href="https://wa.me/573113274403" 
              target="_blank" 
              rel="noopener noreferrer"
              class="font-semibold text-emerald-400 hover:underline flex items-center gap-1.5 mt-0.5"
            >
              <MessageCircle class="w-3.5 h-3.5" />
              <span>+57 311 327 4403</span>
            </a>
          </div>

          <div class="bg-white/5 p-3 rounded-xl border border-white/10">
            <span class="text-slate-400 block text-[11px]">Pastora Martha Escudero</span>
            <a 
              href="https://wa.me/573135094547" 
              target="_blank" 
              rel="noopener noreferrer"
              class="font-semibold text-emerald-400 hover:underline flex items-center gap-1.5 mt-0.5"
            >
              <MessageCircle class="w-3.5 h-3.5" />
              <span>+57 313 509 4547</span>
            </a>
          </div>

          <button
            type="button"
            on:click={scrollToTop}
            class="w-full inline-flex items-center justify-center gap-2 py-2 px-3 rounded-xl bg-white/10 hover:bg-white/20 text-xs font-semibold text-white transition-colors mt-2"
          >
            <ArrowUp class="w-3.5 h-3.5" />
            <span>Volver arriba</span>
          </button>
        </div>
      </div>

    </div>

    <!-- Cita bíblica solemne -->
    <div class="py-6 border-t border-white/10 text-center">
      <blockquote class="font-serif italic text-xs sm:text-sm text-slate-300">
        "Porque donde están dos o tres congregados en mi nombre, allí estoy yo en medio de ellos." — Mateo 18:20
      </blockquote>
    </div>

    <!-- Barra Inferior de Derechos -->
    <div class="pt-6 border-t border-white/5 flex flex-col sm:flex-row items-center justify-between text-xs text-slate-400 gap-2">
      <p>
        © {new Date().getFullYear()} Iglesia Cristiana Nueva Vida • Bremen, Sincelejo, Sucre, Colombia.
      </p>
      <p class="flex items-center gap-1">
        <span>Para la sola gloria de Dios</span>
        <Heart class="w-3 h-3 text-red-400 fill-red-400" />
      </p>
    </div>

  </div>
</footer>

