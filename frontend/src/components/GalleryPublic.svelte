<script lang="ts">
  import { onMount } from 'svelte';
  import { getGalleryItems, deleteGalleryItem } from '../services/admin';
  import type { GalleryItem } from '../types';
  import { Image, Video, Trash2 } from 'lucide-svelte';

  export let adminMode = false;

  let items: GalleryItem[] = [];
  let loading = true;
  let error = '';

  onMount(async () => {
    try {
      items = await getGalleryItems();
      items.sort((a, b) => b.uploaded_at.localeCompare(a.uploaded_at));
    } catch (err) {
      error = 'Error cargando la galería';
      console.error(err);
    } finally {
      loading = false;
    }
  });

  async function handleDelete(id: string) {
    if (!confirm('¿Eliminar este elemento de la galería?')) return;
    try {
      await deleteGalleryItem(id);
      items = items.filter(i => i.id !== id);
    } catch {
      alert('No se pudo eliminar el elemento');
    }
  }
</script>

{#if loading}
  <div class="flex items-center justify-center py-20">
    <div class="animate-spin rounded-full h-8 w-8 border-4 border-church-600 border-t-transparent"></div>
  </div>
{:else if error}
  <div class="bg-red-50 border border-red-200 rounded-xl p-6 text-red-700 text-sm text-center">
    {error}
  </div>
{:else}
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
    {#if items.length === 0}
      <div class="col-span-full text-center py-20">
        <Image class="w-12 h-12 text-slate-400 mx-auto mb-4" />
        <h3 class="font-serif text-lg font-medium text-slate-900 mb-2">Galería vacía</h3>
        <p class="text-slate-500 text-sm">No hay fotos ni videos aún.</p>
        {#if adminMode}
          <p class="text-slate-500 text-sm mt-2">Sube tu primer archivo usando el botón "Subir Archivo"</p>
        {/if}
      </div>
    {/if}
    {#each items as item}
      <div class="bg-white rounded-2xl border border-slate-200 shadow-sm overflow-hidden flex flex-col group">
        <div class="relative aspect-video overflow-hidden bg-slate-100">
          {#if item.file_type === 'image'}
            <img
              src={item.file_url}
              alt={item.title}
              class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
              loading="lazy"
            />
            <Image class="absolute top-2 left-2 w-5 h-5 text-white/90 drop-shadow" />
          {:else}
            <video
              src={item.file_url}
              class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
              poster=""
              preload="none"
            >
              <track kind="captions" />
              Tu navegador no soporta video.
            </video>
            <Video class="absolute top-2 left-2 w-5 h-5 text-blue-400/90 drop-shadow" />
          {/if}
          <div class="absolute top-2 right-2 bg-black/50 text-white px-2 py-1 rounded text-[10px] font-semibold uppercase">
            {item.file_type === 'video' ? 'Video' : 'Foto'}
          </div>
        </div>

        <div class="p-4">
          <h4 class="font-semibold text-slate-900 text-sm line-clamp-1">{item.title}</h4>
          {#if item.description}
            <p class="text-xs text-slate-500 mt-1 line-clamp-2">{item.description}</p>
          {/if}
          <div class="flex items-center justify-between mt-3">
            <span class="text-[10px] font-bold text-church-700 bg-church-50 px-2 py-1 rounded uppercase tracking-wider">
              {item.category}
            </span>
            <span class="text-[10px] text-slate-400">
              {new Date(item.uploaded_at).toLocaleDateString('es-CO')}
            </span>
          </div>
        </div>

        {#if adminMode}
          <div class="px-4 pb-3 flex justify-end">
            <button
              on:click={() => handleDelete(item.id)}
              class="text-slate-400 hover:text-red-500 hover:bg-red-50 p-2 rounded-lg transition-colors"
              title="Eliminar"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}