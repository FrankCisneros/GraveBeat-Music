<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-6">💿 Álbumes</h1>

    <div class="flex gap-2 mb-6">
      <input
        v-model="dir"
        class="input input-bordered flex-1 max-w-2xl"
        placeholder="Ruta de carpeta (ej: C:\\Users\\gungr\\Music)"
      />
      <button class="btn btn-primary" @click="startScan" :disabled="loading">
        {{ loading ? "Escaneando..." : "Escanear" }}
      </button>
    </div>

    <div v-if="error" class="alert alert-error mb-4">
      <span>{{ error }}</span>
    </div>

    <!-- Mostrar álbumes -->
    <div v-if="Object.keys(albums).length" class="space-y-6">
      <div
        v-for="(tracks, albumName) in albums"
        :key="albumName"
        class="p-4 bg-base-100 rounded-lg shadow-sm"
      >
        <!-- Encabezado del álbum -->
        <div class="flex items-center gap-4 mb-4">
          <img
            :src="tracks[0].cover_data_url || 'https://placehold.co/120x120'"
            class="w-20 h-20 object-cover rounded"
            alt="Carátula del álbum"
          />
          <div>
            <h2 class="text-xl font-semibold">{{ albumName }}</h2>
            <p class="text-sm text-gray-500">
              {{ getArtists(tracks).join(', ') }}
            </p>
            <p class="text-xs text-gray-400">
              {{ tracks.length }} canción{{ tracks.length !== 1 ? 'es' : '' }}
            </p>
          </div>
        </div>

        <!-- Lista de canciones -->
        <div class="divide-y divide-base-300">
          <div
            v-for="track in tracks"
            :key="track.path"
            class="flex justify-between items-center py-2 px-2 hover:bg-base-200 rounded transition-colors"
          >
            <div class="truncate flex-1">
              <div class="font-medium truncate">
                {{ track.title || track.name || "Sin título" }}
              </div>
              <div class="text-xs text-gray-500 truncate">
                {{ track.artist || "Artista desconocido" }}
              </div>
            </div>

            <div class="flex items-center gap-2">
              <span class="text-xs text-gray-500">
                {{ formatDuration(track.duration) }}
              </span>
              <button
                class="btn btn-circle btn-primary btn-xs"
                @click="playTrack(track)"
                title="Reproducir"
              >
                ▶
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Estado vacío -->
    <div v-else-if="!loading" class="text-center py-12 text-gray-500">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-16 w-16 mx-auto mb-4 text-gray-300"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
        />
      </svg>
      <p>Ingresa una ruta y haz clic en "Escanear" para agrupar por álbum</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from "vue"
import { invoke } from "@tauri-apps/api/core"

const dir = ref("")
const tracks = ref([])
const loading = ref(false)
const error = ref("")

// Agrupar por álbum (clave: album)
const albums = computed(() => {
  const grouped = {}
  for (const t of tracks.value) {
    const albumName = t.album || "Álbum desconocido"
    if (!grouped[albumName]) grouped[albumName] = []
    grouped[albumName].push(t)
  }
  return grouped
})

async function startScan() {
  if (!dir.value.trim()) {
    error.value = "Por favor, ingresa una ruta válida."
    return
  }

  loading.value = true
  error.value = ""
  tracks.value = []

  try {
    const result = await invoke("scan_folder", { path: dir.value })
    console.log("Archivos escaneados:", result)
    tracks.value = Array.isArray(result) ? result : []
  } catch (err) {
    console.error("Error al escanear carpeta:", err)
    error.value = String(err)
  } finally {
    loading.value = false
  }
}

function getArtists(trackList) {
  // Devuelve una lista única de artistas del álbum
  const artists = new Set(
    trackList.map((t) => t.artist || "Artista desconocido")
  )
  return Array.from(artists)
}

function formatDuration(seconds) {
  if (!seconds || isNaN(seconds)) return "--:--"
  const mins = Math.floor(seconds / 60)
  const secs = Math.floor(seconds % 60)
  return `${mins}:${secs.toString().padStart(2, "0")}`
}

function playTrack(track) {
  alert(
    `Reproduciendo: ${track.title || track.name}\nArtista: ${
      track.artist || "Desconocido"
    }`
  )
}
</script>

<style scoped>
.truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
