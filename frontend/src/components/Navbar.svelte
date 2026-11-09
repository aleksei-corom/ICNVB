<script lang="ts">
  import { Menu, X, Phone, Calendar, HeartHandshake, MapPin, Image, Shield } from 'lucide-svelte';
  import { APP_CONFIG } from '../config';

  export let churchName = "Iglesia Cristiana Nueva Vida";
  export let location = "Bremen, Sincelejo";

  const waBenjamin = APP_CONFIG.waLink(
    APP_CONFIG.contact.whatsapp.benjamin,
    "Hola%20Pastores,%20deseo%20m%C3%A1s%20informaci%C3%B3n%20sobre%20la%20Iglesia%20Cristiana%20Nueva%20Vida"
  );
  const waBenjaminVisit = APP_CONFIG.waLink(
    APP_CONFIG.contact.whatsapp.benjamin,
    "Hola%20Pastor%20Benjam%C3%ADn,%20le%20escribo%20desde%20la%20p%C3%A1gina%20web"
  );
  const waMarthaVisit = APP_CONFIG.waLink(
    APP_CONFIG.contact.whatsapp.martha,
    "Hola%20Pastora%20Martha,%20le%20escribo%20desde%20la%20p%C3%A1gina%20web"
  );

  let isMenuOpen = false;

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
  }

  function closeMenu() {
    isMenuOpen = false;
  }
</script>

<header class="sticky top-0 z-50 bg-white/95 backdrop-blur-md border-b border-slate-200/80 shadow-sm transition-all">
  <!-- Top bar con info rápida -->
  <div class="bg-gradient-to-r from-church-950 via-church-900 to-church-800 text-white text-xs py-1.5 px-4">
    <div class="max-w-7xl mx-auto flex flex-wrap justify-between items-center gap-2">
      <div class="flex items-center gap-3">
        <span class="inline-flex items-center gap-1 text-church-200">
          <MapPin class="w-3.5 h-3.5 text-gold-400" />
          <span>Corregimiento Bremen, Sincelejo, Sucre</span>
        </span>
        <span class="hidden md:inline text-church-400">|</span>
        <span class="hidden md:inline text-slate-200">Culto Dominical: Domingos 9:00 AM</span>
      </div>

      <div class="flex items-center gap-3 text-xs">
        <a 
          href={waBenjamin} 
          target="_blank" 
          rel="noopener noreferrer"
          class="flex items-center gap-1 text-gold-300 hover:text-white transition-colors"
        >
          <Phone class="w-3 h-3" />
          <span>Pastoral: {APP_CONFIG.contact.phone.benjamin}</span>
        </a>
      </div>
    </div>
  </div>

  <!-- Barra Principal de Navegación -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between items-center h-20">
      
      <!-- Logo y Nombre de la Iglesia -->
      <a href="#inicio" class="flex items-center gap-3 group text-left">
        <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-church-700 via-church-800 to-church-950 flex items-center justify-center text-white shadow-md shadow-church-900/20 group-hover:scale-105 transition-transform">
          <!-- Cruz estilizada y paloma -->
          <svg class="w-7 h-7 text-gold-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 2v20M7 7h10" />
            <circle cx="12" cy="7" r="1.5" fill="currentColor" />
          </svg>
        </div>
        <div class="flex flex-col">
          <span class="font-serif font-bold text-xl sm:text-2xl text-church-950 tracking-tight leading-tight group-hover:text-church-700 transition-colors">
            {churchName}
          </span>
          <span class="text-xs uppercase tracking-widest font-semibold text-gold-600">
            {location} • Colombia
          </span>
        </div>
      </a>

      <!-- Menú en Pantallas Grandes -->
      <nav class="hidden lg:flex items-center space-x-1 font-medium text-sm text-slate-700">
        <a href="#inicio" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Inicio</a>
        <a href="#horarios" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Horarios</a>
        <a href="#programacion" class="px-3 py-2 rounded-lg font-semibold text-church-800 bg-church-50 border border-church-200/60 hover:bg-church-100 transition-colors flex items-center gap-1.5">
          <Calendar class="w-4 h-4 text-church-600" />
          Programación Mensual
        </a>
        <a href="#pastores" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Pastores</a>
        <a href="#ministerios" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Ministerios</a>
        <a href="#creemos" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Lo Que Creemos</a>
        <a href="#oracion" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors flex items-center gap-1 text-gold-700">
          <HeartHandshake class="w-4 h-4" />
          Peticiones
        </a>
        <a href="#ubicacion" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors">Ubicación</a>
        <a href="/galeria" class="px-3 py-2 rounded-lg hover:text-church-700 hover:bg-slate-100/70 transition-colors flex items-center gap-1">
          <Image class="w-4 h-4" />
          Galería
        </a>
      </nav>

      <!-- Botón de Acción Principal -->
      <div class="hidden lg:flex items-center gap-3">
        <a 
          href="#oracion" 
          class="inline-flex items-center justify-center px-4 py-2.5 rounded-xl font-semibold text-sm text-white bg-gradient-to-r from-church-700 to-church-900 hover:from-church-800 hover:to-church-950 shadow-md shadow-church-900/20 hover:shadow-lg transition-all transform hover:-translate-y-0.5"
        >
          Pedir Oración
        </a>
      </div>

      <!-- Botón Menú Móvil -->
      <div class="flex lg:hidden items-center">
        <button
          type="button"
          on:click={toggleMenu}
          class="p-2.5 rounded-xl text-slate-700 hover:text-church-800 hover:bg-slate-100 focus:outline-none focus:ring-2 focus:ring-church-500"
          aria-label="Abrir menú"
        >
          {#if isMenuOpen}
            <X class="w-6 h-6" />
          {:else}
            <Menu class="w-6 h-6" />
          {/if}
        </button>
      </div>

    </div>
  </div>

  <!-- Menú Desplegable Móvil -->
  {#if isMenuOpen}
    <div class="lg:hidden border-t border-slate-200 bg-white px-4 pt-3 pb-6 space-y-2 shadow-xl animate-in slide-in-from-top duration-200">
      <a 
        href="#inicio" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        Inicio
      </a>
      <a 
        href="#horarios" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        Horarios de Cultos
      </a>
      <a 
        href="#programacion" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-semibold text-church-800 bg-church-50 border border-church-200"
      >
        📅 Programación Mensual
      </a>
      <a 
        href="#pastores" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        Nuestros Pastores
      </a>
      <a 
        href="#ministerios" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        Ministerios
      </a>
      <a 
        href="#creemos" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        Lo Que Creemos
      </a>
      <a 
        href="#oracion" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-gold-700 hover:bg-gold-50"
      >
        🙏 Petición de Oración
      </a>
      <a 
        href="#ubicacion" 
        on:click={closeMenu}
        class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100"
      >
        📍 Ubicación y Contacto
      </a>

      <div class="pt-3 border-t border-slate-100 flex flex-col gap-2">
        <a 
          href="/galeria" 
          on:click={closeMenu}
          class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100 flex items-center gap-2"
        >
          <Image class="w-4 h-4" />
          Galería de la Iglesia
        </a>
        <a 
          href="/admin/login" 
          on:click={closeMenu}
          class="block px-3 py-2.5 rounded-lg text-base font-medium text-slate-700 hover:bg-slate-100 flex items-center gap-2"
        >
          <Shield class="w-4 h-4" />
          Administración
        </a>
        <div class="pt-3 border-t border-slate-100 flex flex-col gap-2">
        <a 
          href={waBenjaminVisit} 
          target="_blank"
          rel="noopener noreferrer"
          class="w-full text-center py-2.5 px-4 rounded-xl font-medium text-sm text-emerald-800 bg-emerald-50 border border-emerald-200"
        >
          Escribir al Pastor Benjamín
        </a>
        <a 
          href={waMarthaVisit} 
          target="_blank"
          rel="noopener noreferrer"
          class="w-full text-center py-2.5 px-4 rounded-xl font-medium text-sm text-purple-800 bg-purple-50 border border-purple-200"
        >
          Escribir a la Pastora Martha
        </a>
      </div>
    </div>
  </div>
  {/if}
</header>

