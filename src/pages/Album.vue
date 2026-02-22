<template>
  <div class="p-4">
    <!-- <h1 class="text-2xl font-bold mb-6">💿 Álbumes</h1> -->

    <div v-if="error" class="alert alert-error mb-4">
      <span>{{ error }}</span>
    </div>

    <div v-if="Object.keys(albums).length" class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-7 gap-6">

      <div v-for="(albumTracks, albumName) in albums" :key="albumName" class="group cursor-pointer"
        @click="goToAlbum(albumName)">

        <div class="relative aspect-square overflow-hidden rounded-xl shadow-lg mb-3">
          <img :src="coverSrc(albumTracks[0].cover_path)"
            class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-110" alt="Carátula" />

          <div
            class="absolute inset-0 bg-black/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
            <!-- <div class="text-white">
              <span class="text-xl font-bold">Ver Álbum</span>
            </div> -->
          </div>
        </div>

        <div class="px-1">
          <h2 class="font-bold truncate text-base" :title="albumName">
            {{ albumName }}
          </h2>
          <p class="text-sm text-base-content/60 truncate">
            {{ getArtists(albumTracks).join(', ') }}
          </p>
        </div>
      </div>
    </div>

    <div v-else-if="!loading" class="text-center py-12 text-base-content/50">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 opacity-30" fill="none" viewBox="0 0 24 24"
        stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
      </svg>
      <p class="text-lg">No hay álbumes para mostrar.</p>
      <p class="text-sm">Escanea tu biblioteca desde el inicio.</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from "vue"
import { invoke, convertFileSrc } from "@tauri-apps/api/core"

import { useSettingsStore } from "../store/settings"
import { usePlayerStore } from "../store/player"

const tracks = ref([])
const loading = ref(false)
const error = ref("")
const playerStore = usePlayerStore()
const settingsStore = useSettingsStore()

const PLACEHOLDER = "https://placehold.co/300x300?text=No+Cover"

// Agrupar por álbum
const albums = computed(() => {
  const grouped = {}
  for (const t of tracks.value) {
    const albumName = t.album || "Álbum desconocido"
    if (!grouped[albumName]) grouped[albumName] = []
    grouped[albumName].push(t)
  }
  return grouped
})

async function loadTracks() {
  try {
    tracks.value = await invoke("get_tracks", {
      limit: 2000,
      offset: 0,
      profileId: settingsStore.activeProfileId
    })
  } catch {
    error.value = "Error al cargar canciones"
  }
}

watch(() => settingsStore.activeProfileId, loadTracks)

function getArtists(trackList) {
  const artists = new Set(trackList.map((t) => t.artist || "Artista desconocido"))
  return Array.from(artists)
}

function coverSrc(path) {
  return path ? convertFileSrc(path) : PLACEHOLDER
}

import { useRouter } from "vue-router"

const router = useRouter()

function goToAlbum(albumName) {
  router.push(`/album/${encodeURIComponent(albumName)}`)
}

onMounted(loadTracks)
</script>