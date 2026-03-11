<template>
    <div class="h-full flex flex-col p-1 overflow-y-auto">
        <!-- Back Button -->
        <div class="mb-4">
            <button @click="router.back()" class="btn btn-ghost btn-sm gap-2 pl-0 hover:bg-transparent">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                </svg>
                Volver
            </button>
        </div>

        <div v-if="loading" class="flex-1 flex justify-center items-center">
            <span class="loading loading-spinner loading-lg text-primary"></span>
        </div>

        <div v-else-if="songs.length > 0" class="flex flex-col gap-8">
            <!-- Header Section -->
            <div class="flex flex-col md:flex-row gap-8 items-end">
                <!-- Album Cover -->
                <div class="w-52 h-52 rounded-xl overflow-hidden shadow-2xl shrink-0">
                    <img :src="coverSrc(songs[0].cover_path)" class="w-full h-full object-cover" alt="Album Cover">
                </div>

                <!-- Album Info -->
                <div class="flex flex-col gap-4 pb-2 w-full">
                    <div>
                        <h4 class="uppercase text-xs font-bold tracking-widest opacity-70 mb-2">Album</h4>
                        <h1 class="text-4xl md:text-5xl font-black mb-2">{{ albumName }}</h1>
                        <div class="flex items-center gap-2 text-sm opacity-80">
                            <span class="font-bold text-base-content">{{ songs[0].artist || 'Unknown Artist' }}</span>
                            <span>•</span>
                            <span>{{ songs.length }} canciones</span>
                            <span>•</span>
                            <span>{{ totalDuration }}</span>
                        </div>
                    </div>

                    <!-- Action Buttons -->
                    <div class="flex gap-4 mt-2">
                        <button
                            class="btn btn-info btn-lg gap-2 rounded-full px-8 shadow-lg hover:scale-105 transition-transform"
                            @click="playAlbum">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 20 20"
                                fill="currentColor">
                                <path fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                                    clip-rule="evenodd" />
                            </svg>
                            Reproducir
                        </button>
                    </div>
                </div>
            </div>

            <!-- Songs List -->
            <div class="bg-base-100/50 rounded-xl border border-base-200 shadow-inner overflow-hidden">
                <!-- Header for the list -->
                <div
                    class="flex items-center px-4 py-2 text-xs font-semibold text-base-content uppercase tracking-wider border-b border-base-200">
                    <div class="w-8 text-center shrink-0">#</div>
                    <div class="flex-1 pl-4">Título</div>
                    <div class="hidden md:block w-48 text-left pr-4">Artista</div>
                    <div class="w-20 text-right">Duración</div>
                </div>

                <div class="flex flex-col">
                    <SongRow v-for="(song, index) in songs" :key="song.path" :track="song" :index="index"
                        :show-cover="false" :is-active="isCurrentTrack(song)" :is-favorite="song.is_favorite"
                        @play="playSong(song)" @toggleFavorite="toggleFavorite" />
                </div>
            </div>
        </div>

        <div v-else class="flex-1 flex flex-col justify-center items-center opacity-50">
            <h2 class="text-2xl font-bold">No tracks found</h2>
            <p>Este álbum parece estar vacío.</p>
        </div>
    </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import { invoke, convertFileSrc } from "@tauri-apps/api/core"
import { usePlayerStore } from "../store/player"
import { useSettingsStore } from "../store/settings"
import SongRow from "../components/SongRow.vue"

const route = useRoute()
const router = useRouter()
const player = usePlayerStore()
const settings = useSettingsStore()

const loading = ref(true)
const songs = ref([])
const albumName = computed(() => decodeURIComponent(route.params.name || ""))

async function loadSongs() {
    if (!albumName.value) return
    loading.value = true
    try {
        songs.value = await invoke("get_album_songs", {
            profileId: settings.activeProfileId,
            album: albumName.value
        })
    } catch (e) {
        console.error("Failed to load album songs", e)
    } finally {
        loading.value = false
    }
}

const totalDuration = computed(() => {
    const seconds = songs.value.reduce((acc, curr) => acc + (curr.duration || 0), 0)
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)

    if (h > 0) return `${h} h ${m} min`
    return `${m} min ${Math.floor(seconds % 60)} s`
})

function coverSrc(path) {
    return path ? convertFileSrc(path) : "https://placehold.co/300x300?text=No+Cover"
}

function playAlbum() {
    player.setPlaylist(songs.value)
    player.play(songs.value[0])
}

function playSong(track) {
    player.setPlaylist(songs.value)
    player.play(track)
}

function isCurrentTrack(track) {
    return player.currentTrack?.path === track.path
}

async function toggleFavorite(track) {
    if (track.is_favorite) {
        await invoke("remove_favorite", {
            songId: track.id,
            profileId: settings.activeProfileId
        })
        track.is_favorite = false
    } else {
        await invoke("add_favorite", {
            songId: track.id,
            profileId: settings.activeProfileId
        })
        track.is_favorite = true
    }
    // force reactivity if needed
    songs.value = [...songs.value]
}

onMounted(loadSongs)

watch(() => route.params.name, loadSongs)
</script>
