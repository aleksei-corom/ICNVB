<script lang="ts">
  import { adminLogin } from '../services/admin';
  import { HeartHandshake, Shield, ArrowRight, Church } from 'lucide-svelte';

  let username = '';
  let password = '';
  let error = '';
  let isSubmitting = false;

  async function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    isSubmitting = true;

    try {
      await adminLogin({ username: username.trim(), password });
      window.location.href = '/admin';
    } catch {
      error = 'Usuario o contraseña incorrectos';
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="min-h-screen flex items-center justify-center bg-slate-50">
  <div class="max-w-md w-full space-y-8 bg-white p-8 sm:p-10 rounded-3xl shadow-lg border border-slate-200">
    <div class="text-center">
      <div class="w-16 h-16 rounded-2xl bg-church-800 text-white flex items-center justify-center mx-auto mb-4">
        <Church class="w-8 h-8" />
      </div>
      <h2 class="font-serif text-3xl font-bold text-slate-900 mb-2">
        Iglesia Cristiana Nueva Vida
      </h2>
      <p class="text-slate-600 text-sm">
        Panel de Administración • Bremen, Sincelejo
      </p>
    </div>

    <form on:submit|preventDefault={handleLogin} class="mt-8 space-y-6">
      {#if error}
        <div class="p-4 rounded-xl bg-red-50 border border-red-200 text-red-700 text-sm flex items-center gap-3">
          <Shield class="w-5 h-5 shrink-0" />
          <span>{error}</span>
        </div>
      {/if}

      <div class="space-y-4">
        <label for="username" class="block text-sm font-semibold text-slate-700">
          Usuario
        </label>
        <input
          id="username"
          type="text"
          bind:value={username}
          required
          class="w-full px-4 py-3 rounded-xl bg-slate-50 border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 focus:border-church-500 transition-all"
          placeholder="admin"
        />
      </div>

      <div class="space-y-4">
        <label for="password" class="block text-sm font-semibold text-slate-700">
          Contraseña
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          required
          class="w-full px-4 py-3 rounded-xl bg-slate-50 border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 focus:border-church-500 transition-all"
          placeholder="••••••••"
        />
      </div>

      <button
        type="submit"
        disabled={isSubmitting}
        class="w-full flex justify-center py-4 px-4 rounded-xl border border-transparent bg-church-800 text-white font-semibold text-sm sm:text-base hover:bg-church-900 disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-md shadow-church-900/20"
      >
        {#if isSubmitting}
          <div class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent mr-2"></div>
          Iniciando sesión...
        {:else}
          <ArrowRight class="w-4 h-4 mr-2" />
          Iniciar Sesión
        {/if}
      </button>

      <div class="text-xs text-slate-500 text-center">
        Usuario: admin • Contraseña: icnvb2026
      </div>
    </form>
  </div>
</div>

<style>
  input {
    font-size: 1rem;
    line-height: 1.5rem;
  }
</style>
