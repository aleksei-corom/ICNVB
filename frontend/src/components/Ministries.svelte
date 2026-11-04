<script lang="ts">
  import { Flame, Sparkles, Smile, HeartHandshake, Music, Compass, ArrowRight } from 'lucide-svelte';

  const ministries = [
    {
      title: "Jóvenes y Jovencitos",
      schedule: "Viernes 7:00 PM",
      icon: Flame,
      iconColor: "text-amber-600 bg-amber-100",
      description: "Un movimiento apasionado por Cristo. Formamos jóvenes firmes en la fe, con valores cristianos, líderes comprometidos y testimonio íntegro en sus escuelas, universidades y familias.",
      tag: "Viernes 7:00 PM",
    },
    {
      title: "Ministerio de Damas (Mujeres de Fe)",
      schedule: "Jueves 7:00 PM",
      icon: Sparkles,
      iconColor: "text-pink-600 bg-pink-100",
      description: "Mujeres valientes, sabias y de oración que edifican sus hogares bajo el consejo bíblico. Liderado por nuestra amada Pastora Martha Escudero.",
      tag: "Jueves 7:00 PM",
    },
    {
      title: "Escuela Bíblica Infantil (Semillitas de Dios)",
      schedule: "Domingos 9:00 AM",
      icon: Smile,
      iconColor: "text-emerald-600 bg-emerald-100",
      description: "Atención amorosa y divertida para los niños durante el culto dominical. Clases bíblicas creativas, manualidades, cantos y memorización de versículos bíblicos.",
      tag: "Domingos 9:00 AM",
    },
    {
      title: "Intercesión & Ayuno Congregacional",
      schedule: "Jueves 9:00 AM",
      icon: HeartHandshake,
      iconColor: "text-church-600 bg-church-100",
      description: "La columna vertebral de la iglesia: un pueblo de rodillas clamando por sanidades, milagros, restauración familiar y salvación en el corregimiento de Bremen.",
      tag: "Jueves 9:00 AM",
    },
    {
      title: "Alabanza & Adoración",
      schedule: "En todos los cultos",
      icon: Music,
      iconColor: "text-purple-600 bg-purple-100",
      description: "Músicos y adoradores entregados a exaltar al Rey de Reyes en espíritu y en verdad, preparando una atmósfera propicia para la gloria de Dios.",
      tag: "Alabanza en Vivo",
    },
    {
      title: "Evangelismo & Acción Social",
      schedule: "Sábados y actividades de campo",
      icon: Compass,
      iconColor: "text-teal-600 bg-teal-100",
      description: "Llevamos el mensaje del Evangelio y el amor práctico de Jesucristo a las veredas y barrios vecinos de Bremen, Sincelejo y Sucre.",
      tag: "Misiones Locales",
    },
  ];
</script>

<section id="ministerios" class="py-20 bg-white relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <div class="text-center max-w-3xl mx-auto mb-16">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
        <Sparkles class="w-3.5 h-3.5 text-church-600" />
        <span>Servicio y Crecimiento</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Nuestros Ministerios
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        En la Iglesia Cristiana Nueva Vida cada persona tiene un lugar para servir, conectarse y crecer espiritualmente según los dones que el Señor le ha concedido.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each ministries as min}
        <div class="bg-slate-50/60 rounded-3xl p-7 border border-slate-200/90 hover:bg-white hover:shadow-xl transition-all duration-300 flex flex-col justify-between group hover:-translate-y-1">
          <div>
            <div class="flex items-center justify-between mb-5">
              <div class="w-14 h-14 rounded-2xl {min.iconColor} flex items-center justify-center shadow-inner group-hover:scale-110 transition-transform">
                <svelte:component this={min.icon} class="w-7 h-7" />
              </div>
              <span class="text-xs font-semibold px-3 py-1 rounded-full bg-white border border-slate-200 text-slate-600">
                {min.tag}
              </span>
            </div>

            <h3 class="font-serif font-bold text-xl text-slate-900 mb-3 group-hover:text-church-700 transition-colors">
              {min.title}
            </h3>

            <p class="text-slate-600 text-sm leading-relaxed mb-6">
              {min.description}
            </p>
          </div>

          <div class="pt-4 border-t border-slate-200/60 flex items-center justify-between">
            <span class="text-xs font-medium text-slate-500">Horario habitual</span>
            <span class="text-xs font-bold text-church-800">{min.schedule}</span>
          </div>
        </div>
      {/each}
    </div>

    <!-- Banner motivacional -->
    <div class="mt-14 text-center">
      <p class="text-slate-600 text-sm">
        ¿Tienes un talento musical, pasión por la enseñanza infantil o anhelo de servir?
        <a 
          href="https://wa.me/573113274403?text=Hola%20Pastores,%20me%20gustar%C3%ADa%20servir%20en%20uno%20de%20los%20ministerios%20de%20la%20iglesia" 
          target="_blank"
          rel="noopener noreferrer"
          class="font-bold text-church-700 hover:text-church-900 underline ml-1"
        >
          Habla con los pastores aquí
        </a>
      </p>
    </div>

  </div>
</section>

