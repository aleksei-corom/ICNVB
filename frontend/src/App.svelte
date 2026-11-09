<script lang="ts">
  import { onMount } from 'svelte';
  import Navbar from './components/Navbar.svelte';
  import Hero from './components/Hero.svelte';
  import WeeklySchedule from './components/WeeklySchedule.svelte';
  import MonthlyProgram from './components/MonthlyProgram.svelte';
  import Pastors from './components/Pastors.svelte';
  import Ministries from './components/Ministries.svelte';
  import Doctrine from './components/Doctrine.svelte';
  import PrayerRequest from './components/PrayerRequest.svelte';
  import LocationContact from './components/LocationContact.svelte';
  import Footer from './components/Footer.svelte';
  import AdminLogin from './components/AdminLogin.svelte';
  import AdminDashboard from './components/AdminDashboard.svelte';
  import GalleryPublic from './components/GalleryPublic.svelte';

  let currentPath = window.location.pathname;

  function handleRouteChange() {
    currentPath = window.location.pathname;
  }

  onMount(() => {
    window.addEventListener('popstate', handleRouteChange);
    return () => window.removeEventListener('popstate', handleRouteChange);
  });

  function isAdminRoute() {
    return currentPath.startsWith('/admin');
  }

  function isGalleryRoute() {
    return currentPath === '/galeria';
  }

  function isHomeRoute() {
    return currentPath === '/' || currentPath === '';
  }
</script>

<div class="min-h-screen flex flex-col bg-slate-50 text-slate-800 antialiased selection:bg-church-600 selection:text-white">

  {#if isAdminRoute()}
    <!-- Rutas de Administración -->
    {#if currentPath === '/admin/login'}
      <AdminLogin />
    {:else if currentPath.startsWith('/admin')}
      <AdminDashboard />
    {/if}
  {:else if isGalleryRoute()}
    <!-- Galería Pública -->
    <div class="min-h-screen py-12 px-4 sm:px-6 lg:px-8">
      <div class="max-w-7xl mx-auto">
        <div class="text-center mb-12">
          <div class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-church-100 text-church-800 text-xs font-bold uppercase tracking-wider mb-3">
            <Image class="w-3 h-3" />
            <span>Galería de la Iglesia</span>
          </div>
          <h1 class="font-serif text-3xl sm:text-4xl font-bold text-slate-900 tracking-tight mb-4">
            Galería de Fotos y Videos
          </h1>
          <p class="text-slate-600 text-lg">
            Recuerdos de las actividades de la Iglesia Cristiana Nueva Vida en Bremen, Sincelejo.
          </p>
        </div>
        <GalleryPublic adminMode={false} />
      </div>
    </div>
  {:else}
    <!-- Sitio público -->
    <Navbar />
    <main class="flex-grow">
      <Hero />
      <WeeklySchedule />
      <MonthlyProgram />
      <Pastors />
      <Ministries />
      <Doctrine />
      <PrayerRequest />
      <LocationContact />
    </main>
    <Footer />
  {/if}
</div>

<script lang="ts" context="module">
  import { Image } from 'lucide-svelte';
</script>
