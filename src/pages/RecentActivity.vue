<template>
    <div class="h-full flex flex-col p-4 bg-base-100/50">
        <div class="flex items-end gap-4 mb-6">
            <h1 class="text-3xl font-bold">Actividad Reciente</h1>
            <p class="text-sm opacity-50 mb-1">Canciones escuchadas en esta sesión</p>
        </div>

        <div v-if="songs.length === 0" class="flex flex-col items-center justify-center p-20 text-base-content/50">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <p>No has reproducido ninguna canción todavía</p>
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
                    <SongRow v-for="(song, index) in songs" :key="index + '-' + song.path" :track="song" :index="index"
                        :is-active="playerStore.currentTrack?.path === song.path" :is-favorite="song.is_favorite"
                        @play="playSong(song)" @toggleFavorite="toggleFavorite(song)" />
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { computed } from 'vue';
import { usePlayerStore } from '../store/player';
import { useSettingsStore } from '../store/settings';
import { invoke } from '@tauri-apps/api/core';
import SongRow from '../components/SongRow.vue';

const playerStore = usePlayerStore();
const settingsStore = useSettingsStore();

const songs = computed(() => playerStore.recentTracks);

const playSong = (song) => {
    playerStore.setPlaylist([...songs.value]);
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
    // No need to copy array, refs array items mutability works
};
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
