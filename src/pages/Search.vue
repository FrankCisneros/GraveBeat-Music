<template>
    <div class="h-full flex flex-col p-4 bg-base-100/50">
        <h1 class="text-3xl font-bold mb-6">Resultados para: "{{ searchQuery }}"</h1>

        <div v-if="loading" class="flex justify-center p-10">
            <span class="loading loading-spinner loading-lg"></span>
        </div>

        <div v-else-if="songs.length === 0" class="flex flex-col items-center justify-center p-20 text-base-content/50">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <p>No se encontraron canciones</p>
        </div>

        <div v-else class="flex-1 overflow-y-auto custom-scroll pr-2">
            <div class="bg-base-100/50 rounded-xl border border-base-200 shadow-inner overflow-hidden">
                <div
                    class="flex items-center px-4 py-2 text-xs font-semibold text-base-content uppercase tracking-wider border-b border-base-200">
                    <div class="w-8 text-center shrink-0">#</div>
                    <div class="flex-1 pl-4 md:pl-14">Título</div>
                    <div class="hidden md:block w-48 text-left pr-4">Artista</div>
                    <div class="hidden lg:block w-48 text-left pr-4">Álbum</div>
                    <div class="w-20 text-right">Duración</div>
                </div>
                <div class="flex flex-col">
                    <SongRow v-for="(song, index) in songs" :key="song.path" :track="song" :index="index"
                        :is-active="playerStore.currentTrack?.path === song.path" :is-favorite="song.is_favorite"
                        @play="playSong(song)" @toggleFavorite="toggleFavorite(song)" />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { usePlayerStore } from '../store/player';
import { useSettingsStore } from '../store/settings';
import SongRow from '../components/SongRow.vue';

const route = useRoute();
const playerStore = usePlayerStore();
const settingsStore = useSettingsStore();

const searchQuery = ref('');
const songs = ref([]);
const loading = ref(false);

const loadResults = async () => {
    searchQuery.value = route.query.q || '';
    if (!searchQuery.value) {
        songs.value = [];
        return;
    }

    loading.value = true;
    try {
        songs.value = await invoke('search_tracks', {
            profileId: settingsStore.activeProfileId,
            query: searchQuery.value
        });
    } catch (e) {
        console.error(e);
    } finally {
        loading.value = false;
    }
};

const playSong = (song) => {
    playerStore.play(song);
};

const toggleFavorite = async (track) => {
    if (track.is_favorite) {
        await invoke("remove_favorite", {
            songId: track.id,
            profileId: settingsStore.activeProfileId
        });
        track.is_favorite = false;
    } else {
        await invoke("add_favorite", {
            songId: track.id,
            profileId: settingsStore.activeProfileId
        });
        track.is_favorite = true;
    }
    songs.value = [...songs.value];
};

onMounted(() => {
    loadResults();
});

watch(() => route.query.q, () => {
    loadResults();
});
</script>

<style scoped>
.custom-scroll::-webkit-scrollbar {
    width: 6px;
}

.custom-scroll::-webkit-scrollbar-thumb {
    background-color: var(--fallback-bc, oklch(var(--bc)/0.2));
    border-radius: 10px;
}
</style>
