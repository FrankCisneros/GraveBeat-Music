<template>
    <div class="h-full flex flex-col p-6">
        <!-- Encabezado de la Playlist -->
        <div class="flex items-end gap-6 mb-8 shrink-0">
            <div
                class="w-48 h-48 bg-base-300 rounded-xl shadow-2xl flex items-center justify-center overflow-hidden shrink-0 group">
                <!-- Se podría poner portada de la primera canción, o icono -->
                <svg xmlns="http://www.w3.org/2000/svg" class="w-20 h-20 text-base-content/20" fill="none"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
                </svg>
            </div>

            <div class="flex flex-col pb-2">
                <span class="text-sm font-bold tracking-widest uppercase text-base-content/60 mb-2">Playlist
                    Pública</span>
                <h1 class="text-5xl font-black mb-4 truncate">{{ playlist?.name || 'Cargando...' }}</h1>
                <p class="text-sm text-base-content/60">{{ tracks.length }} canciones</p>
            </div>

            <div class="ml-auto pb-2 flex gap-2">
                <button class="btn btn-error btn-sm" @click="deletePlaylist">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    Eliminar Playlist
                </button>
            </div>
        </div>

        <div v-if="error" class="alert alert-error mb-4 shrink-0">
            <span>{{ error }}</span>
        </div>

        <!-- Lista de Canciones -->
        <div v-if="tracks.length"
            class="flex-1 min-h-0 bg-base-100/50 rounded-xl border border-base-200 shadow-inner overflow-hidden">
            <div
                class="flex items-center px-4 py-2 text-xs font-semibold text-base-content/50 uppercase tracking-wider border-b border-base-200">
                <div class="w-8 text-center shrink-0">#</div>
                <div class="flex-1 pl-14">Título / Artista</div>
                <div class="hidden md:block w-32 text-right pr-4">Álbum</div>
                <div class="w-20 text-right">Duración</div>
                <div class="w-10"></div> <!-- Options -->
            </div>

            <RecycleScroller class="scroller h-full pr-1" :items="tracks" :item-size="64" key-field="path"
                v-slot="{ item: track, index }">
                <div class="relative group flex w-full">
                    <SongRow class="flex-1" :track="track" :index="index" :is-active="currentTrack?.path === track.path"
                        :is-favorite="track.is_favorite" @play="playTrack(track)" />
                    <!-- Delete from playlist button overlay -->
                    <button
                        class="absolute right-14 top-1/2 -translate-y-1/2 btn btn-ghost btn-xs btn-circle text-error opacity-0 group-hover:opacity-100 transition-opacity"
                        title="Quitar de la Playlist" @click.stop="removeSong(track)">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
            </RecycleScroller>
        </div>

        <div v-else-if="!loading" class="flex-1 flex flex-col items-center justify-center text-base-content/50">
            <p class="text-lg font-medium">Esta playlist está vacía.</p>
            <p class="text-sm mt-2">Agrega canciones usando el Menú Contextual (los 3 puntos) sobre cualquier canción.
            </p>
        </div>

        <div v-else class="flex-1 flex items-center justify-center">
            <span class="loading loading-spinner text-primary loading-lg"></span>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../store/settings'
import { usePlayerStore } from '../store/player'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import SongRow from '../components/SongRow.vue'

const route = useRoute()
const router = useRouter()
const settingsStore = useSettingsStore()
const playerStore = usePlayerStore()

const playlistId = computed(() => Number(route.params.id))
const playlist = ref(null)
const tracks = ref([])
const loading = ref(false)
const error = ref('')

const currentTrack = computed(() => playerStore.currentTrack)

async function loadData() {
    loading.value = true
    try {
        const pId = playlistId.value
        // Find playlist details
        const playlists = await invoke("get_playlists", { profileId: settingsStore.activeProfileId })
        playlist.value = playlists.find(p => p.id === pId) || null

        if (playlist.value) {
            tracks.value = await invoke("get_playlist_songs", {
                playlistId: pId,
                profileId: settingsStore.activeProfileId
            })
        }
    } catch (err) {
        console.error(err)
        error.value = "Error al cargar la playlist."
    } finally {
        loading.value = false
    }
}

async function deletePlaylist() {
    if (confirm("¿Seguro que quieres eliminar esta playlist?")) {
        try {
            await invoke("delete_playlist", { playlistId: playlistId.value })
            router.push("/library")
        } catch (err) {
            alert("Error al eliminar la playlist")
        }
    }
}

async function removeSong(track) {
    try {
        await invoke("remove_song_from_playlist", {
            playlistId: playlistId.value,
            songId: track.id
        })
        tracks.value = tracks.value.filter(t => t.id !== track.id)
    } catch (err) {
        console.error("No se pudo remover", err)
    }
}

function playTrack(track) {
    playerStore.setPlaylist(tracks.value)
    playerStore.play(track)
}

onMounted(() => {
    loadData()
})

watch(() => route.params.id, () => {
    loadData()
})
watch(() => settingsStore.activeProfileId, () => {
    loadData()
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
