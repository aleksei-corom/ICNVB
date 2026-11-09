<script lang="ts">
  import { uploadGalleryFile } from '../services/admin';
  import { APP_CONFIG } from '../config';
  import { Upload, Image, Video } from 'lucide-svelte';

  export let onClose: () => void = () => {};

  let title = '';
  let description = '';
  let category = 'Otro';
  let fileType = 'image';
  let selectedFile: File | null = null;
  let isUploading = false;
  let error = '';
  let previewUrl: string | null = null;

  const categories = APP_CONFIG.galleryCategories;

  function handleFileChange(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      selectedFile = input.files[0];
      previewUrl = URL.createObjectURL(selectedFile);
      fileType = selectedFile.type.startsWith('video') ? 'video' : 'image';
    }
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    error = '';
    isUploading = true;

    if (!selectedFile) {
      error = 'Selecciona un archivo';
      isUploading = false;
      return;
    }

    const formData = new FormData();
    formData.append('title', title.trim());
    formData.append('description', description.trim() || '');
    formData.append('file_type', fileType);
    formData.append('file_name', selectedFile.name);
    formData.append('category', category.trim());
    formData.append('file', selectedFile);

    try {
      await uploadGalleryFile(formData);
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Error subiendo el archivo';
    } finally {
      isUploading = false;
    }
  }

  function removeFile() {
    selectedFile = null;
    previewUrl = null;
    const input = document.getElementById('gallery-file-input') as HTMLInputElement;
    if (input) input.value = '';
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-5">
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label for="gallery-title" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Título *
      </label>
      <input
        id="gallery-title"
        type="text"
        bind:value={title}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
        placeholder="Ej. Culto Dominical - Noviembre 2026"
      />
    </div>

    <div>
      <label for="gallery-category" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
        Categoría *
      </label>
      <select
        id="gallery-category"
        bind:value={category}
        required
        class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      >
        {#each categories as cat}
          <option value={cat}>{cat}</option>
        {/each}
      </select>
    </div>
  </div>

  <div>
    <label for="gallery-description" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Descripción
    </label>
    <textarea
      id="gallery-description"
      bind:value={description}
      rows="2"
      class="w-full px-4 py-3 rounded-xl bg-white border border-slate-200 text-slate-900 focus:outline-none focus:ring-2 focus:ring-church-500 transition-all"
      placeholder="Breve descripción de la actividad..."
    ></textarea>
  </div>

  <div>
    <label for="gallery-file-input" class="block text-xs font-semibold text-slate-600 uppercase tracking-wider mb-1.5">
      Archivo *
    </label>
    <div class="border-2 border-dashed border-slate-300 rounded-xl p-6 text-center hover:border-church-300 hover:bg-slate-50 transition-colors relative">
      <input
        id="gallery-file-input"
        type="file"
        accept="image/*,video/*"
        on:change={handleFileChange}
        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
      />
      {#if previewUrl}
        <div class="relative">
          {#if fileType === 'image'}
            <img src={previewUrl} alt="Preview" class="max-h-48 object-cover rounded-lg mx-auto" />
          {:else}
            <video src={previewUrl} alt="Preview" class="max-h-48 object-cover rounded-lg mx-auto" controls muted>
              <track kind="captions" />
            </video>
          {/if}
          <button
            type="button"
            on:click={removeFile}
            class="absolute -top-3 -right-3 bg-red-500 text-white rounded-full p-1.5 hover:bg-red-600 transition-colors"
            title="Eliminar archivo"
          >
            <span class="sr-only">Eliminar</span>
            ✕
          </button>
        </div>
      {:else}
        <div class="flex flex-col items-center gap-2">
          <Upload class="w-8 h-8 text-slate-400" />
          <p class="text-sm text-slate-600">Haz clic para seleccionar una imagen o video</p>
          <p class="text-xs text-slate-500">Formatos: JPG, PNG, MP4, MOV (máx. 10MB)</p>
        </div>
      {/if}
    </div>
  </div>

  {#if error}
    <div class="p-3 rounded-xl bg-red-50 border border-red-200 text-red-700 text-sm">
      {error}
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
      disabled={isUploading || !selectedFile}
      class="px-6 py-2 rounded-xl font-semibold text-white bg-church-800 hover:bg-church-900 disabled:opacity-50 transition-all"
    >
      {#if isUploading}
        Subiendo...
      {:else}
        Subir a Galería
      {/if}
    </button>
  </div>
</form>