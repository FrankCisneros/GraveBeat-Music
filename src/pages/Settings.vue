<template>
  <div class="p-6 max-w-4xl mx-auto">
    <h1 class="text-3xl font-bold mb-8">Settings</h1>

    <div class="space-y-8">
      <!-- Library Section -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title mb-4">Library Management</h2>
          
          <div class="form-control w-full">
            <label class="label">
              <span class="label-text">Music Folder</span>
            </label>
            <div class="flex gap-4">
              <input type="text" :value="musicPath" placeholder="No folder selected" class="input input-bordered w-full" readonly />
              <button @click="selectFolder" class="btn btn-primary">
                Select Folder
              </button>
            </div>
            <label class="label">
              <span class="label-text-alt">Selected folder will be scanned for music files</span>
            </label>
          </div>

          <div class="divider"></div>

          <div class="flex justify-between items-center">
            <div>
              <h3 class="font-bold">Clear Database</h3>
              <p class="text-sm opacity-70">Remove all indexed songs and playlists</p>
            </div>
            <button @click="handleClearDB" class="btn btn-error btn-outline">
              Clear Database
            </button>
          </div>
        </div>
      </div>

      <!-- Appearance Section -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title mb-4">Appearance</h2>
          
          <div class="form-control w-full max-w-xs">
            <label class="label">
              <span class="label-text">Theme</span>
            </label>
            <select v-model="temaActual" class="select select-bordered focus:border-0">
              <option v-for="t in temas" :key="t" :value="t">{{ t }}</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Data Management -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title mb-4">Data Management</h2>
          
          <div class="flex justify-between items-center">
            <div>
              <h3 class="font-bold">Export Playlists</h3>
              <p class="text-sm opacity-70">Save your playlists to a JSON file</p>
            </div>
            <button @click="exportPlaylists" class="btn btn-secondary">
              Export Playlists
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useTheme } from '../composables/useTheme';
import { useLibrary } from '../composables/useLibrary';

const { temas, temaActual } = useTheme();
const { musicPath, setMusicFolder, clearDatabase } = useLibrary();

async function selectFolder() {
  await setMusicFolder();
}

async function handleClearDB() {
}

function exportPlaylists() {

}
</script>
