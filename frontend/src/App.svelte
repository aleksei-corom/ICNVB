<script lang="ts">
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
</script>

<div class="min-h-screen flex flex-col bg-slate-50 text-slate-800 antialiased selection:bg-church-600 selection:text-white">
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
</div>

