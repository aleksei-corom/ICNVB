<script lang="ts">
  import { ShieldCheck, BookOpen, Cross, Flame, Users, Globe } from 'lucide-svelte';

  const beliefs = [
    {
      title: "La Santa Biblia",
      verse: "2 Timoteo 3:16-17",
      icon: BookOpen,
      iconColor: "text-church-600 bg-church-50",
      description: "Creemos que la Biblia es la Palabra inspirada e inerrante de Dios, la revelación completa de Su voluntad para la salvación del hombre y la máxima autoridad para la fe y la conducta cristiana.",
    },
    {
      title: "El Dios Trino y Eterno",
      verse: "1 Juan 5:7; Mateo 28:19",
      icon: ShieldCheck,
      iconColor: "text-gold-600 bg-gold-50",
      description: "Creemos en un solo Dios viviente, verdadero y eterno, creador de los cielos y la tierra, existente en tres personas divinas: Padre, Hijo y Espíritu Santo.",
    },
    {
      title: "Salvación Únicamente en Jesucristo",
      verse: "Efesios 2:8-9; Juan 14:6",
      icon: Cross,
      iconColor: "text-red-600 bg-red-50",
      description: "Creemos que la salvación no se obtiene por obras humanas, sino por pura gracia mediante la fe en el sacrificio expiatorio de Jesucristo en la cruz y Su gloriosa resurrección.",
    },
    {
      title: "El Poder y Guía del Espíritu Santo",
      verse: "Hechos 1:8; Juan 16:13",
      icon: Flame,
      iconColor: "text-amber-600 bg-amber-50",
      description: "Creemos en la presencia activa del Espíritu Santo, quien regenera al pecador, consuela, santifica al creyente y capacita a la Iglesia con dones espirituales para el testimonio y la edificación.",
    },
    {
      title: "La Iglesia y la Vida en Comunidad",
      verse: "Hebreos 10:24-25; Hechos 2:42",
      icon: Users,
      iconColor: "text-emerald-600 bg-emerald-50",
      description: "Creemos que la Iglesia local es el cuerpo de Cristo en la tierra, llamada a congregarse en amor, celebrar los sacramentos ordenados (Bautismo y Santa Cena) y crecer en fraternidad.",
    },
    {
      title: "La Gran Comisión y la Esperanza Bendita",
      verse: "Mateo 28:19-20; Tito 2:13",
      icon: Globe,
      iconColor: "text-sky-600 bg-sky-50",
      description: "Creemos en la misión de proclamar las Buenas Nuevas a toda persona en Bremen, Sincelejo y las naciones, y aguardamos la segunda venida de nuestro Señor y Salvador Jesucristo.",
    },
  ];
</script>

<section id="creemos" class="py-20 bg-slate-50 relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <div class="text-center max-w-3xl mx-auto mb-16">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
        <ShieldCheck class="w-3.5 h-3.5 text-church-600" />
        <span>Fundamentos Doctrinales</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Lo Que Creemos
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        Nuestra fe está anclada firmemente en las Sagradas Escrituras y el evangelio de la gracia de Dios manifestado en Cristo Jesús.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 lg:gap-8">
      {#each beliefs as item}
        <div class="bg-white rounded-3xl p-7 border border-slate-200/90 shadow-sm hover:shadow-lg transition-all duration-300 flex flex-col justify-between">
          <div>
            <div class="flex items-center justify-between mb-4">
              <div class="w-12 h-12 rounded-2xl {item.iconColor} flex items-center justify-center shadow-inner">
                <svelte:component this={item.icon} class="w-6 h-6" />
              </div>
              <span class="text-xs font-serif italic text-gold-700 bg-gold-50 border border-gold-200 px-2.5 py-1 rounded-lg">
                {item.verse}
              </span>
            </div>

            <h3 class="font-serif font-bold text-lg text-slate-900 mb-2">
              {item.title}
            </h3>

            <p class="text-slate-600 text-sm leading-relaxed">
              {item.description}
            </p>
          </div>
        </div>
      {/each}
    </div>

  </div>
</section>

