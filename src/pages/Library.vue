<template>
    <div class="h-full flex flex-col">
        <!-- <div class="flex items-center justify-between mb-6 shrink-0">
            <h1 class="text-2xl font-bold">🎶 Biblioteca local</h1>
        </div> -->

        <div v-if="error" class="alert alert-error mb-4 shrink-0">
            <span>{{ error }}</span>
        </div>

        <div v-if="tracks.length"
            class="flex-1 min-h-0 bg-base-100/50 rounded-xl border border-base-200 shadow-inner overflow-hidden">
            <!-- Header for the list -->
            <div
                class="flex items-center px-4 py-2 text-xs font-semibold text-base-content/50 uppercase tracking-wider border-b border-base-200">
                <div class="w-8 text-center shrink-0">#</div>
                <div class="flex-1 pl-14">Título / Artista</div>
                <div class="hidden md:block w-32 text-right pr-4">Álbum</div>
                <div class="w-20 text-right">Duración</div>
            </div>

            <RecycleScroller class="scroller h-full pr-1" :items="tracks" :item-size="64" key-field="path"
                v-slot="{ item: track, index }">
                <SongRow :track="track" :index="index" :is-active="currentTrack?.path === track.path"
                    :is-favorite="track.is_favorite" @play="playTrack(track)" @toggleFavorite="toggleFavorite(track)" />
            </RecycleScroller>

            <div class="mt-2 text-center text-xs text-gray-500 shrink-0 pb-2">
                Total: {{ tracks.length }} canciones cargadas
            </div>
        </div>

        <div v-else-if="!loading" class="flex-1 flex flex-col items-center justify-center text-base-content/50">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4 opacity-50" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
            </svg>
            <p class="text-lg font-medium">Tu biblioteca está vacía.</p>
            <p class="text-sm mt-2">Agrega carpetas en Configuración y presiona Escanear.</p>
        </div>

        <div v-else class="flex-1 flex items-center justify-center">
            <span class="loading loading-spinner loading-lg text-primary"></span>
        </div>
    </div>
</template>

<script setup>
import { onMounted, ref, shallowRef, watch, computed, triggerRef } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { useSettingsStore } from "../store/settings"
import { usePlayerStore } from "../store/player"
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import SongRow from '../components/SongRow.vue'

const tracks = shallowRef([])
const loading = ref(false)
const error = ref("")
const settingsStore = useSettingsStore()
const playerStore = usePlayerStore()

const currentTrack = computed(() => playerStore.currentTrack)

async function loadTracks() {
    loading.value = true
    try {
        const data = await invoke("get_tracks", {
            limit: 10000,
            offset: 0,
            profileId: settingsStore.activeProfileId
        })
        tracks.value = data
        // Only set playlist if it's empty or we want to sync it? 
        // Typically we might not want to overwrite the playlist immediately on load unless requested.
        // But for "Library" view usually clicking a song sets the scope to Library.
    } catch (err) {
        console.error(err)
        error.value = "Error al cargar canciones"
    } finally {
        loading.value = false
    }
}

function playTrack(track) {
    playerStore.setPlaylist(tracks.value)
    playerStore.play(track)
}

async function toggleFavorite(track) {
    if (track.is_favorite) {
        await invoke("remove_favorite", {
            songId: track.id,
            profileId: settingsStore.activeProfileId
        })
        track.is_favorite = false
    } else {
        await invoke("add_favorite", {
            songId: track.id,
            profileId: settingsStore.activeProfileId
        })
        track.is_favorite = true
    }
    triggerRef(tracks)
}

onMounted(async () => {
    loadTracks()
})

watch(() => settingsStore.activeProfileId, async () => {
    loadTracks()
})
</script>

<style scoped>
.scroller::-webkit-scrollbar {
    width: 6px;
}

.scroller::-webkit-scrollbar-track {
    background: transparent;
}

.scroller::-webkit-scrollbar-thumb {
    background: #4b5563;
    border-radius: 10px;
}

.scroller::-webkit-scrollbar-thumb:hover {
    background: #6b7280;
}
</style>