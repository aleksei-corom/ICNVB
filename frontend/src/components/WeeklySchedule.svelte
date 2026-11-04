<script lang="ts">
  import { Sun, BookOpen, HeartHandshake, Sparkles, Flame, Moon, Clock, Share2, Check } from 'lucide-svelte';

  let copiedIndex: number | null = null;

  const schedules = [
    {
      day: "Domingos",
      time: "9:00 AM - 11:30 AM",
      badge: "Culto Principal",
      badgeColor: "bg-church-100 text-church-800 border-church-200",
      title: "Culto General & Escuela Dominical",
      description: "Celebración y adoración congregacional con alabanza en vivo, mensaje de la Palabra de Dios para la vida diaria y salones de enseñanza bíblica por edades para los niños.",
      audience: "Toda la familia (Niños, Jóvenes y Adultos)",
      icon: Sun,
      iconBg: "bg-amber-100 text-amber-600",
    },
    {
      day: "Miércoles",
      time: "7:00 PM - 9:00 PM",
      badge: "Doctrina & Clamor",
      badgeColor: "bg-sky-100 text-sky-800 border-sky-200",
      title: "Culto de Oración & Enseñanza Bíblica",
      description: "Profundizamos en los principios de las Sagradas Escrituras, fortaleciendo el carácter cristiano y dedicando un tiempo especial de intercesión unida.",
      audience: "Congregación y Amigos visitantes",
      icon: BookOpen,
      iconBg: "bg-sky-100 text-sky-600",
    },
    {
      day: "Jueves (Mañana)",
      time: "9:00 AM - 12:00 M",
      badge: "Búsqueda Profunda",
      badgeColor: "bg-emerald-100 text-emerald-800 border-emerald-200",
      title: "Ayuno Congregacional de Intercesión",
      description: "Consagración personal y colectiva, humillación ante el Señor y clamor ferviente por las necesidades de salud, familias y salvación en Bremen y Sincelejo.",
      audience: "Creyentes en ayuno e intercesores",
      icon: HeartHandshake,
      iconBg: "bg-emerald-100 text-emerald-600",
    },
    {
      day: "Jueves (Noche)",
      time: "7:00 PM - 9:00 PM",
      badge: "Ministerio Femenino",
      badgeColor: "bg-pink-100 text-pink-800 border-pink-200",
      title: "Culto de Damas (Mujeres de Fe)",
      description: "Espacio diseñado para edificar a las mujeres, madres, esposas y jóvenes con temas bíblicos prácticos, testimonios de bendición y comunión íntima.",
      audience: "Damas, Madres y Señoritas",
      icon: Sparkles,
      iconBg: "bg-pink-100 text-pink-600",
    },
    {
      day: "Viernes",
      time: "7:00 PM - 9:00 PM",
      badge: "Juventud en Fuego",
      badgeColor: "bg-orange-100 text-orange-800 border-orange-200",
      title: "Culto de Jóvenes y Jovencitos",
      description: "Reunión vibrante y participativa para adolescentes y jóvenes con música, devocionales contemporáneos, dinámicas y confraternidad sana en Cristo.",
      audience: "Adolescentes y Jóvenes",
      icon: Flame,
      iconBg: "bg-orange-100 text-orange-600",
    },
    {
      day: "Mensual",
      time: "Viernes por la noche (Fecha mensual)",
      badge: "Avivamiento",
      badgeColor: "bg-indigo-100 text-indigo-800 border-indigo-200",
      title: "Gran Vigilia de Oración y Avivamiento",
      description: "Noche de búsqueda sobrenatural del Espíritu Santo, alabanza extendida y oración de madrugada. Se anuncia puntualmente en la programación mensual.",
      audience: "Toda la congregación y pueblo sediento de Dios",
      icon: Moon,
      iconBg: "bg-indigo-100 text-indigo-600",
    },
  ];

  function shareSchedule(item: typeof schedules[0], idx: number) {
    const text = `Te invito a la Iglesia Cristiana Nueva Vida en Bremen, Sincelejo:
*${item.title}*
📅 ${item.day}
⏰ ${item.time}
📍 Corregimiento Bremen, Sincelejo, Sucre.
¡Te esperamos con los brazos abiertos!`;
    const url = `https://api.whatsapp.com/send?text=${encodeURIComponent(text)}`;
    window.open(url, '_blank');
    copiedIndex = idx;
    setTimeout(() => { copiedIndex = null; }, 3000);
  }
</script>

<section id="horarios" class="py-20 bg-slate-50 relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <!-- Encabezado de Sección -->
    <div class="text-center max-w-3xl mx-auto mb-16">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
        <Clock class="w-3.5 h-3.5 text-church-600" />
        <span>Nuestros Encuentros Semanales</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Horarios de Cultos y Reuniones
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        Tenemos un lugar reservado para ti y tu familia en cada una de nuestras actividades durante la semana. ¡Acompáñanos a glorificar a Dios!
      </p>
    </div>

    <!-- Rejilla de Horarios -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8">
      {#each schedules as item, index}
        <div class="bg-white rounded-2xl p-6 sm:p-7 shadow-sm hover:shadow-xl border border-slate-200/80 transition-all duration-300 flex flex-col justify-between group hover:-translate-y-1">
          <div>
            <!-- Cabecera de tarjeta -->
            <div class="flex items-center justify-between mb-4">
              <div class="w-12 h-12 rounded-xl {item.iconBg} flex items-center justify-center group-hover:scale-110 transition-transform shadow-inner">
                <svelte:component this={item.icon} class="w-6 h-6" />
              </div>
              <span class="px-2.5 py-1 rounded-full text-xs font-semibold border {item.badgeColor}">
                {item.badge}
              </span>
            </div>

            <!-- Día y Hora -->
            <div class="mb-3">
              <span class="text-xs uppercase font-bold text-slate-500 tracking-wider block">
                {item.day}
              </span>
              <div class="text-lg font-bold text-church-900 flex items-center gap-1.5 mt-0.5">
                <Clock class="w-4 h-4 text-church-600" />
                <span>{item.time}</span>
              </div>
            </div>

            <!-- Título y Descripción -->
            <h3 class="font-serif font-bold text-xl text-slate-900 mb-2 group-hover:text-church-700 transition-colors">
              {item.title}
            </h3>
            <p class="text-slate-600 text-sm leading-relaxed mb-4">
              {item.description}
            </p>
          </div>

          <!-- Pie de tarjeta con Audiencia y Botón Compartir -->
          <div class="pt-4 border-t border-slate-100 flex items-center justify-between">
            <span class="text-xs text-slate-500 font-medium">
              Dirigido a: <strong class="text-slate-700">{item.audience}</strong>
            </span>
            <button
              type="button"
              on:click={() => shareSchedule(item, index)}
              class="inline-flex items-center gap-1 text-xs font-semibold text-church-600 hover:text-church-800 bg-church-50 hover:bg-church-100 px-2.5 py-1.5 rounded-lg transition-colors"
              title="Invitar por WhatsApp"
            >
              {#if copiedIndex === index}
                <Check class="w-3.5 h-3.5 text-emerald-600" />
                <span class="text-emerald-700">¡Abriendo!</span>
              {:else}
                <Share2 class="w-3.5 h-3.5" />
                <span>Invitar</span>
              {/if}
            </button>
          </div>
        </div>
      {/each}
    </div>

    <!-- Nota Informativa adicional -->
    <div class="mt-12 max-w-2xl mx-auto bg-gradient-to-r from-church-900 to-church-950 text-white p-6 rounded-2xl shadow-md text-center">
      <h4 class="font-serif font-bold text-lg text-gold-300 mb-1">
        ¿Es tu primera vez visitándonos?
      </h4>
      <p class="text-slate-300 text-sm mb-4">
        No necesitas vestimenta formal obligatoria ni pagar ninguna entrada. Ven tal como eres, nuestro equipo de ujieres y los pastores te recibirán con alegría.
      </p>
      <a 
        href="https://wa.me/573113274403?text=Hola%20Pastores,%20estar%C3%A9%20visitando%20la%20iglesia%20este%20fin%20de%20semana"
        target="_blank"
        rel="noopener noreferrer"
        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl font-semibold text-xs sm:text-sm bg-gold-400 text-church-950 hover:bg-gold-300 transition-colors"
      >
        <span>Avisar de mi visita por WhatsApp</span>
      </a>
    </div>

  </div>
</section>

