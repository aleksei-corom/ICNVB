<script lang="ts">
  import { MapPin, Phone, MessageCircle, Navigation, HelpCircle, Check, Compass } from 'lucide-svelte';
  import { APP_CONFIG } from '../config';

  const waLocation = APP_CONFIG.waLink(
    APP_CONFIG.contact.whatsapp.benjamin,
    "Hola%20Pastor%20Benjam%C3%ADn,%20%C2%BFme%20podr%C3%ADa%20indicar%20la%20ubicaci%C3%B3n%20exacta%20en%20Bremen%20para%20visitarles?"
  );

  const faqs = [
    {
      q: "¿Cómo debo vestir para asistir?",
      a: "No exigimos ningún tipo de ropa formal. Ven cómodo, con respeto y sencillez. Lo que más le interesa al Señor es la disposición de tu corazón.",
    },
    {
      q: "¿Tienen un lugar especial para los niños?",
      a: "¡Sí! Todos los domingos contamos con la Escuela Bíblica Infantil (Semillitas de Dios), donde maestras capacitadas enseñan valores y la Palabra de Dios a través de juegos y dinámicas.",
    },
    {
      q: "¿Tiene algún costo la entrada a los cultos?",
      a: "De ninguna manera. La entrada es totalmente libre y gratuita para todos los cultos, vigilias y reuniones de la iglesia.",
    },
    {
      q: "¿Cómo puedo solicitar consejería pastoral?",
      a: "Puedes contactar directamente a los Pastores Benjamín Ramos (+57 311 327 4403) o Martha Escudero (+57 313 509 4547) por llamada o WhatsApp para coordinar un espacio.",
    },
  ];
</script>

<section id="ubicacion" class="py-20 bg-slate-50 relative">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    
    <div class="text-center max-w-3xl mx-auto mb-16">
      <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
        <MapPin class="w-3.5 h-3.5 text-church-600" />
        <span>Encuéntranos</span>
      </div>
      <h2 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
        Ubicación y Planifica tu Visita
      </h2>
      <p class="text-slate-600 text-base sm:text-lg">
        Estamos ubicados en el corregimiento de Bremen, en la zona rural de Sincelejo, departamento de Sucre. ¡Será una bendición recibirte!
      </p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-10 items-start">
      
      <!-- Información de Dirección y Contacto -->
      <div class="space-y-6">
        
        <div class="bg-white rounded-3xl p-8 border border-slate-200/90 shadow-sm space-y-6">
          <div class="flex items-start gap-4">
            <div class="w-12 h-12 rounded-2xl bg-church-100 text-church-700 flex items-center justify-center shrink-0">
              <MapPin class="w-6 h-6" />
            </div>
            <div>
              <h3 class="font-serif font-bold text-xl text-slate-900 mb-1">
                Dirección Principal
              </h3>
              <p class="text-slate-700 text-sm leading-relaxed">
                Templo Central Iglesia Cristiana Nueva Vida<br />
                <strong>Corregimiento Bremen</strong>, Municipio de Sincelejo<br />
                Departamento de Sucre, Colombia.
              </p>
            </div>
          </div>

          <div class="flex items-start gap-4 pt-4 border-t border-slate-100">
            <div class="w-12 h-12 rounded-2xl bg-emerald-100 text-emerald-700 flex items-center justify-center shrink-0">
              <Compass class="w-6 h-6" />
            </div>
            <div>
              <h3 class="font-serif font-bold text-base text-slate-900 mb-1">
                Cómo llegar desde Sincelejo
              </h3>
              <p class="text-slate-600 text-xs sm:text-sm leading-relaxed">
                Desde el casco urbano de Sincelejo puedes tomar transporte hacia el corregimiento de Bremen. Al ingresar al centro poblado de Bremen, pregunta por la Iglesia Cristiana Nueva Vida de los pastores Benjamín y Martha.
              </p>
            </div>
          </div>

          <!-- Botones de Acción Inmediata -->
          <div class="pt-4 border-t border-slate-100 flex flex-col sm:flex-row gap-3">
            <a
              href={waLocation} 
              target="_blank"
              rel="noopener noreferrer"
              class="flex-1 inline-flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-xs sm:text-sm font-semibold text-emerald-800 bg-emerald-50 hover:bg-emerald-100 border border-emerald-200 transition-colors"
            >
              <MessageCircle class="w-4 h-4 text-emerald-600" />
              <span>Pedir ubicación por WhatsApp</span>
            </a>

            <a
              href="https://www.google.com/maps/search/?api=1&query=Bremen+Sincelejo+Sucre+Colombia"
              target="_blank"
              rel="noopener noreferrer"
              class="inline-flex items-center justify-center gap-2 py-3 px-4 rounded-xl text-xs sm:text-sm font-semibold text-church-800 bg-church-50 hover:bg-church-100 border border-church-200 transition-colors"
            >
              <Navigation class="w-4 h-4 text-church-600" />
              <span>Ver en Google Maps</span>
            </a>
          </div>
        </div>

        <!-- Preguntas Frecuentes de Visitantes -->
        <div class="bg-white rounded-3xl p-8 border border-slate-200/90 shadow-sm">
          <div class="flex items-center gap-2 text-church-800 font-serif font-bold text-lg mb-4">
            <HelpCircle class="w-5 h-5 text-church-600" />
            <span>Preguntas Frecuentes para Visitantes</span>
          </div>

          <div class="space-y-4">
            {#each faqs as faq}
              <div class="p-3.5 rounded-2xl bg-slate-50 border border-slate-100">
                <h4 class="font-semibold text-xs sm:text-sm text-slate-800 mb-1">
                  {faq.q}
                </h4>
                <p class="text-xs text-slate-600 leading-relaxed">
                  {faq.a}
                </p>
              </div>
            {/each}
          </div>
        </div>

      </div>

      <!-- Mapa Integrado y Visual -->
      <div class="bg-white rounded-3xl p-4 sm:p-6 border border-slate-200/90 shadow-sm flex flex-col h-full">
        <div class="mb-4 flex items-center justify-between">
          <span class="text-xs font-bold uppercase tracking-wider text-slate-500">
            Mapa Geográfico • Sucre, Colombia
          </span>
          <span class="px-2.5 py-0.5 rounded-full text-[10px] font-bold bg-church-100 text-church-800">
            Bremen, Sincelejo
          </span>
        </div>

        <div class="w-full h-80 sm:h-96 rounded-2xl overflow-hidden border border-slate-200 shadow-inner relative">
          <!-- OpenStreetMap iframe centrado en Bremen / Sincelejo -->
          <iframe
            title="Mapa Bremen Sincelejo"
            width="100%"
            height="100%"
            frameborder="0"
            scrolling="no"
            marginheight="0"
            marginwidth="0"
            src="https://www.openstreetmap.org/export/embed.html?bbox=-75.4500%2C9.2500%2C-75.3200%2C9.3500&amp;layer=mapnik&amp;marker=9.2980%2C-75.3900"
            class="w-full h-full"
          ></iframe>
        </div>

        <div class="mt-4 p-4 rounded-2xl bg-amber-50/80 border border-amber-200 text-xs text-amber-900 flex items-start gap-2.5">
          <Navigation class="w-4 h-4 text-amber-700 shrink-0 mt-0.5" />
          <p>
            <strong>Nota para visitas:</strong> Si viajas en vehículo particular o moto desde Sincelejo, la ruta hacia Bremen está señalizada. Puedes comunicarte con los pastores al llegar para guiarte directamente hasta el templo.
          </p>
        </div>
      </div>

    </div>

  </div>
</section>

