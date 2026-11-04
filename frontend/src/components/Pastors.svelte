<script lang="ts">
  import { Phone, MessageCircle, Heart, User, Sparkles, BookOpen } from 'lucide-svelte';

  const pastors = [
    {
      name: "Pastor Benjamín Ramos",
      role: "Pastor Principal",
      phone: "+57 311 327 4403",
      rawPhone: "573113274403",
      greeting: "¡Dios te bendiga grandemente! Mi corazón y el de mi esposa están comprometidos con pastorear con amor y fidelidad a las familias de Bremen y Sincelejo. Estamos aquí para servirte y orar por tu vida.",
      gradient: "from-church-800 to-church-950",
      accent: "text-church-700",
      avatarBg: "bg-church-100 text-church-800 border-church-200",
    },
    {
      name: "Pastora Martha Escudero",
      role: "Pastora & Líder de Ministerio Femenino",
      phone: "+57 313 509 4547",
      rawPhone: "573135094547",
      greeting: "Bienvenido a nuestra amada congregación. Creemos firmemente que Dios tiene un propósito precioso para tu hogar, tus hijos y tu vida. Recuerda que no estás solo: Cristo tiene cuidado de ti.",
      gradient: "from-slate-900 to-church-900",
      accent: "text-pink-700",
      avatarBg: "bg-pink-100 text-pink-800 border-pink-200",
    },
  ];
</script>

<section id="pastores" class="py-20 bg-slate-50 relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <!-- Encabezado de Sección -->
    <div class="text-center max-w-3xl mx-auto mb-16">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
        <Heart class="w-3.5 h-3.5 text-church-600" />
        <span>Liderazgo y Cuidado Pastoral</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Nuestros Pastores
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        Guiados por el Espíritu Santo y la Palabra de Dios, nuestros pastores dedican sus vidas al servicio de la obra en el corregimiento de Bremen, Sincelejo y sus alrededores.
      </p>
    </div>

    <!-- Tarjetas de los Pastores -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-5xl mx-auto">
      {#each pastors as pastor}
        <div class="bg-white rounded-3xl p-8 border border-slate-200/90 shadow-sm hover:shadow-xl transition-all duration-300 flex flex-col justify-between group hover:-translate-y-1">
          <div>
            <!-- Cabecera de la Tarjeta con Avatar -->
            <div class="flex items-center gap-4 mb-6">
              <div class="w-20 h-20 rounded-2xl {pastor.avatarBg} border-2 flex items-center justify-center font-serif text-2xl font-bold shadow-md group-hover:scale-105 transition-transform">
                <User class="w-10 h-10" />
              </div>
              <div>
                <span class="text-xs uppercase font-bold tracking-wider text-gold-700 block mb-1">
                  {pastor.role}
                </span>
                <h3 class="font-serif font-bold text-2xl text-slate-900">
                  {pastor.name}
                </h3>
                <span class="text-xs text-slate-500">Iglesia Cristiana Nueva Vida • Bremen</span>
              </div>
            </div>

            <!-- Mensaje Pastoral -->
            <div class="relative bg-slate-50 rounded-2xl p-5 border border-slate-100 mb-6 italic text-slate-700 text-sm leading-relaxed font-serif">
              <span class="absolute -top-3 left-4 text-3xl text-church-300 leading-none">“</span>
              <p class="relative z-10 pt-1">
                {pastor.greeting}
              </p>
            </div>
          </div>

          <!-- Contacto Directo -->
          <div class="pt-4 border-t border-slate-100 flex flex-col sm:flex-row items-center gap-3">
            <a
              href="https://wa.me/{pastor.rawPhone}?text=Hola%20{encodeURIComponent(pastor.name)},%20le%20escribo%20con%20un%20saludo%20fraternal%20desde%20la%20p%C3%A1gina%20web"
              target="_blank"
              rel="noopener noreferrer"
              class="w-full sm:flex-1 inline-flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-xs sm:text-sm font-semibold text-emerald-800 bg-emerald-50 hover:bg-emerald-100 border border-emerald-200 transition-colors"
            >
              <MessageCircle class="w-4 h-4 text-emerald-600" />
              <span>WhatsApp Pastoral</span>
            </a>

            <a
              href="tel:{pastor.phone.replace(/[^0-9+]/g, '')}"
              class="w-full sm:w-auto inline-flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-xs sm:text-sm font-semibold text-slate-700 bg-slate-100 hover:bg-slate-200 border border-slate-200 transition-colors"
              title="Llamar directamente"
            >
              <Phone class="w-4 h-4" />
              <span>{pastor.phone}</span>
            </a>
          </div>

        </div>
      {/each}
    </div>

    <!-- Cita bíblica de exhortación pastoral -->
    <div class="mt-14 max-w-3xl mx-auto text-center">
      <div class="inline-flex items-center gap-2 text-church-900 bg-church-50 border border-church-200/80 px-6 py-3 rounded-2xl text-xs sm:text-sm font-serif italic shadow-sm">
        <BookOpen class="w-4 h-4 text-church-700 shrink-0" />
        <span>"Y os daré pastores según mi corazón, que os apacienten con ciencia y con inteligencia." — Jeremías 3:15</span>
      </div>
    </div>

  </div>
</section>

