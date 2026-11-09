<script lang="ts">
  import { changePassword } from '../services/admin';
  import type { ChangePasswordRequest } from '../types';
  import { Shield, Lock, X, CheckCircle2, AlertCircle } from 'lucide-svelte';

  export let onClose: () => void = () => {};

  let currentPassword = '';
  let newPassword = '';
  let confirmPassword = '';
  let isSubmitting = false;
  let error = '';
  let success = false;

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    success = false;
    isSubmitting = true;

    if (newPassword !== confirmPassword) {
      error = 'Las contraseñas no coinciden';
      isSubmitting = false;
      return;
    }

    try {
      await changePassword({
        current_password: currentPassword,
        new_password: newPassword,
      });
      success = true;
      currentPassword = '';
      newPassword = '';
      confirmPassword = '';

      setTimeout(() => {
        onClose();
      }, 1500);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Error cambiando contraseña';
    } finally {
      isSubmitting = false;
    }
  }
</script>

{#if success}
  <div class="bg-emerald-50 border border-emerald-200 rounded-xl p-4 text-emerald-700 flex items-start gap-3 mb-4">
    <CheckCircle2 class="w-5 h-5 shrink-0 mt-0.5" />
    <div>
      <h4 class="font-semibold text-emerald-900 mb-1">Contraseña cambiada</h4>
      <p class="text-xs text-emerald-600">Tu contraseña se ha actualizado exitosamente.</p>
    </div>
  </div>
{/if}

{#if error}
  <div class="bg-red-50 border border-red-200 rounded-xl p-4 text-red-700 flex items-start gap-3 mb-4">
    <AlertCircle class="w-5 h-5 shrink-0 mt-0.5" />
    <p class="text-sm">{error}</p>
  </div>
{/if}

<form on:submit|preventDefault={handleSubmit} class="space-y-4">
  <div>
    <label for="change-current-password" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Contraseña actual *
    </label>
    <input
      id="change-current-password"
      type="password"
      bind:value={currentPassword}
      required
      class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      placeholder="••••••••"
    />
  </div>

  <div>
    <label for="change-new-password" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Nueva contraseña *
    </label>
    <input
      id="change-new-password"
      type="password"
      bind:value={newPassword}
      required
      minlength="6"
      class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      placeholder="••••••••"
    />
  </div>

  <div>
    <label for="change-confirm-password" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Confirmar nueva contraseña *
    </label>
    <input
      id="change-confirm-password"
      type="password"
      bind:value={confirmPassword}
      required
      minlength="6"
      class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      placeholder="••••••••"
    />
  </div>

  {#if newPassword !== confirmPassword && confirmPassword}
    <div class="p-3 rounded-xl bg-red-50 border border-red-200 text-red-700 text-sm">
      Las contraseñas no coinciden
    </div>
  {/if}

  <div class="flex justify-end gap-3 pt-4 border-t border-slate-100">
    <button
      type="button"
      on:click={onClose}
      class="px-4 py-2 rounded-xl text-sm font-medium text-slate-600 bg-white border border-slate-200 hover:bg-slate-50 transition-colors"
    >
      Cancelar
    </button>
    <button
      type="submit"
      disabled={isSubmitting || !currentPassword || !newPassword || !confirmPassword || newPassword !== confirmPassword}
      class="px-6 py-2 rounded-xl font-semibold text-white bg-church-800 hover:bg-church-900 disabled:opacity-50 transition-all"
    >
      {#if isSubmitting}
        Cambiando...
      {:else}
        Cambiar Contraseña
      {/if}
    </button>
  </div>
</form>