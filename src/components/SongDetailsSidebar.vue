<template>
    <transition name="slide-right">
        <aside v-if="visible"
            class="fixed inset-y-0 right-0 z-50 w-80 bg-base-100 shadow-2xl border-l border-base-300 flex flex-col transform transition-transform duration-300 ease-in-out font-sans overflow-hidden my-18">

            <!-- Header -->
            <div class="flex items-center justify-between p-4 border-b border-base-200 shrink-0">
                <h3 class="font-bold text-lg">Detalles</h3>
                <button @click="$emit('close')" class="btn btn-ghost btn-sm btn-circle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            <div class="flex-1 overflow-y-auto custom-scroll p-6" v-if="track">
                <!-- Cover Art -->
                <div class="aspect-square w-full rounded-xl overflow-hidden shadow-xl mb-6 relative group">
                    <img :src="coverSrc"
                        class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                        alt="Cover" />
                </div>

                <!-- Track Info -->
                <div class="mb-8 text-center">
                    <h2 class="text-2xl font-bold mb-1 leading-tight">{{ track.title || 'Sin Título' }}</h2>
                    <p class="text-lg text-primary font-medium mb-1">{{ track.artist || 'Artista Desconocido' }}</p>
                    <p class="text-sm text-base-content/60">{{ track.album }}</p>
                </div>

                <!-- Lyrics Section -->
                <div class="bg-base-200/50 rounded-xl p-4">
                    <h4 class="text-xs font-bold uppercase tracking-widest text-base-content/50 mb-4 text-center">Letra
                    </h4>

                    <div v-if="loadingLyrics" class="flex justify-center p-4">
                        <span class="loading loading-spinner text-primary"></span>
                    </div>

                    <div v-else-if="parsedLyrics.length > 0" class="flex flex-col items-center space-y-4">
                        <div v-for="(line, index) in parsedLyrics" :key="index"
                            class="text-center transition-all duration-300 px-4 py-2 rounded-lg" :class="{
                                'text-primary font-bold text-xl scale-110 shadow-sm bg-base-100/50': activeLyricIndex === index,
                                'text-base-content/60 font-medium text-base': activeLyricIndex !== index,
                                'opacity-40': activeLyricIndex !== -1 && index < activeLyricIndex - 2,
                                'opacity-60': activeLyricIndex !== -1 && (index === activeLyricIndex - 2 || index === activeLyricIndex - 1)
                            }" :ref="el => { if (el) lyricRefs[index] = el; }">
                            {{ line.text }}
                        </div>
                    </div>

                    <div v-else-if="lyrics"
                        class="whitespace-pre-wrap text-center leading-relaxed opacity-80 font-medium font-sans">
                        {{ lyrics }}
                    </div>

                    <div v-else class="text-center py-8 opacity-40">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 mx-auto mb-2 opacity-50" fill="none"
                            viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
                        </svg>
                        <p class="text-sm">No hay letra disponible</p>
                    </div>
                </div>
            </div>

            <div v-else class="flex-1 flex items-center justify-center text-base-content/50">
                <p>No hay canción reproduciéndose</p>
            </div>

        </aside>
    </transition>
</template>

<script setup>
import { computed, watch, ref, nextTick } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import axios from 'axios';
import { usePlayerStore } from '../store/player';
import { parseLyrics, getActiveLyricIndex } from '../utils/lyricsSync';

const props = defineProps({
    visible: Boolean,
    track: Object
});

defineEmits(['close']);

const playerStore = usePlayerStore();
const PLACEHOLDER = "https://placehold.co/300x300?text=No+Cover";
const lyrics = ref(null);
const parsedLyrics = ref([]);
const activeLyricIndex = ref(-1);
const lyricRefs = ref({});
const loadingLyrics = ref(false);

const coverSrc = computed(() => {
    return props.track?.cover_path ? convertFileSrc(props.track.cover_path) : PLACEHOLDER;
});

const fetchLyrics = async () => {
    if (!props.track) return;

    lyrics.value = null;
    loadingLyrics.value = true;

    try {
        // 1. Try local file
        const localLyrics = await invoke('get_lyrics', { path: props.track.path });
        if (localLyrics) {
            lyrics.value = localLyrics;
            return;
        }

        // 2. Try LRCLib
        const response = await axios.get('https://lrclib.net/api/get', {
            params: {
                artist_name: props.track.artist,
                track_name: props.track.title,
                album_name: props.track.album,
                duration: props.track.duration
            }
        });

        if (response.data && (response.data.syncedLyrics || response.data.plainLyrics)) {
            const rawLyrics = response.data.syncedLyrics || response.data.plainLyrics;
            lyrics.value = rawLyrics;
            parsedLyrics.value = parseLyrics(rawLyrics);
            if (parsedLyrics.value.length === 0 && rawLyrics) {
                // Not synced
                lyrics.value = rawLyrics;
            } else {
                lyrics.value = null; // Hide plain lyrics if synced exist
            }
        } else {
            lyrics.value = "No se encontró la letra.";
            parsedLyrics.value = [];
        }

    } catch (e) {
        console.warn("Lyrics fetch failed", e);
        lyrics.value = "No se pudo cargar la letra.";
        parsedLyrics.value = [];
    } finally {
        loadingLyrics.value = false;
    }
};

watch(() => props.track, (newTrack) => {
    if (newTrack && props.visible) {
        fetchLyrics();
    }
}, { immediate: true });

watch(() => props.visible, (isVisible) => {
    if (isVisible && props.track && !lyrics.value && parsedLyrics.value.length === 0) {
        fetchLyrics();
    }
});

watch(() => playerStore.currentTime, (time) => {
    if (parsedLyrics.value.length > 0 && props.visible) {
        const newIndex = getActiveLyricIndex(parsedLyrics.value, time);
        if (newIndex !== activeLyricIndex.value) {
            activeLyricIndex.value = newIndex;
            // Scroll to the active lyric
            if (newIndex !== -1 && lyricRefs.value[newIndex]) {
                nextTick(() => {
                    lyricRefs.value[newIndex].scrollIntoView({
                        behavior: 'smooth',
                        block: 'center'
                    });
                });
            }
        }
    }
});
</script>

<style scoped>
.slide-right-enter-active,
.slide-right-leave-active {
    transition: transform 0.3s ease;
}

.slide-right-enter-from,
.slide-right-leave-to {
    transform: translateX(100%);
}

.custom-scroll::-webkit-scrollbar {
    width: 4px;
}

.custom-scroll::-webkit-scrollbar-thumb {
    background: #52525b;
    border-radius: 4px;
}

.custom-scroll::-webkit-scrollbar-thumb:hover {
    background: #71717a;
}
</style>
