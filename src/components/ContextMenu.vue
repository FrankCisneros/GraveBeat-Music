<template>
    <div v-if="uiStats.visible"
        class="fixed z-50 bg-base-200 border border-base-300 shadow-xl rounded-lg overflow-hidden w-56 flex flex-col py-1"
        :style="{ top: uiStats.y + 'px', left: uiStats.x + 'px' }" @click.stop v-click-outside="close">

        <div class="px-4 py-2 border-b border-base-content/10 mb-1">
            <p class="text-xs font-bold uppercase tracking-wider opacity-50 truncate">
                {{ uiStats.track?.title || 'Opciones' }}
            </p>
        </div>

        <button
            class="text-left px-4 py-2 hover:bg-primary hover:text-primary-content text-sm transition-colors flex items-center gap-2"
            @click="action('playNext')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            Reproducir siguiente
        </button>

        <!-- Add to Playlist Submenu -->
        <div class="relative group">
            <button
                class="w-full text-left px-4 py-2 hover:bg-primary hover:text-primary-content text-sm transition-colors flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                    </svg>
                    Agregar a Playlist
                </div>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
            </button>
            <div
                class="absolute top-0 left-full ml-1 w-48 bg-base-200 border border-base-300 shadow-xl rounded-lg overflow-hidden hidden group-hover:block z-50 max-h-64 overflow-y-auto custom-scroll">
                <div v-if="playlists.length === 0" class="px-4 py-2 text-xs opacity-50">
                    No hay playlists
                </div>
                <button v-for="pl in playlists" :key="pl.id"
                    class="w-full text-left px-4 py-2 hover:bg-primary hover:text-primary-content text-sm transition-colors truncate"
                    @click.stop="addToPlaylist(pl.id)">
                    {{ pl.name }}
                </button>
            </div>
        </div>

        <div class="divider my-0"></div>

        <button
            class="text-left px-4 py-2 hover:bg-primary hover:text-primary-content text-sm transition-colors flex items-center gap-2"
            @click="action('showDetails')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            Ver detalles
        </button>

        <button
            class="text-left px-4 py-2 hover:bg-primary hover:text-primary-content text-sm transition-colors flex items-center gap-2"
            @click="action('scanLyrics')">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Buscar Letra
        </button>
    </div>
</template>

<script setup>
import { computed, watch, ref } from 'vue';
import { useUIStore } from '../store/ui';
import { usePlayerStore } from '../store/player';
import { useSettingsStore } from '../store/settings';
import { invoke } from '@tauri-apps/api/core';

const uiStore = useUIStore();
const playerStore = usePlayerStore();
const settingsStore = useSettingsStore();

const uiStats = computed(() => uiStore.contextMenu);
const playlists = ref([]);

// Fetch playlists when menu opens
watch(() => uiStats.value.visible, async (isVisible) => {
    if (isVisible) {
        try {
            playlists.value = await invoke("get_playlists", { profileId: settingsStore.activeProfileId });
        } catch (e) {
            console.error("Error loading playlists for context menu", e);
        }
    }
});

const addToPlaylist = async (playlistId) => {
    const track = uiStats.value.track;
    if (!track) return;
    try {
        await invoke("add_song_to_playlist", {
            playlistId: playlistId,
            songId: track.id
        });
        // Success indicator could be added here
    } catch (e) {
        console.error("Error adding song to playlist", e);
        alert("Esa canción ya está en la playlist o hubo un error.");
    }
    close();
};

const action = (type) => {
    const track = uiStats.value.track;

    switch (type) {
        case 'playNext':
            // Implement queue logic
            console.log("Play Next", track);
            break;
        case 'addToQueue':
            console.log("Queue", track);
            break;
        case 'showDetails':
            if (playerStore.currentTrack?.path !== track.path) {
                // If not playing, keep playing current but show details?
                // Or maybe the Sidebar is for *Playing* track.
                // Let's assume we want to inspect this track.
                // For now, let's just log or maybe change selection?
            }
            // Toggle details view
            playerStore.isDetailsVisible = true;
            // Hack: If we want to see details of a non-playing track, we might need a separate 'selectedTrack' in store or just sidebar mode.
            // For now User asked for "details", let's assume playing track or standard view.
            break;
        case 'scanLyrics':
            playerStore.isDetailsVisible = true;
            // Trigger lyrics fetch
            break;
    }
    close();
};

const close = () => {
    uiStore.hideContextMenu();
};

// Directive for click outside
const vClickOutside = {
    mounted(el, binding) {
        el.clickOutsideEvent = function (event) {
            if (!(el === event.target || el.contains(event.target))) {
                binding.value(event);
            }
        };
        document.addEventListener('click', el.clickOutsideEvent, true); // Capture phase
        document.addEventListener('contextmenu', el.clickOutsideEvent, true);
    },
    unmounted(el) {
        document.removeEventListener('click', el.clickOutsideEvent, true);
        document.removeEventListener('contextmenu', el.clickOutsideEvent, true);
    },
};
</script>

<style scoped>
.custom-scroll::-webkit-scrollbar {
    width: 4px;
}

.custom-scroll::-webkit-scrollbar-track {
    background: transparent;
}

.custom-scroll::-webkit-scrollbar-thumb {
    background: #52525b;
    border-radius: 4px;
}

.custom-scroll::-webkit-scrollbar-thumb:hover {
    background: #71717a;
}
</style>
