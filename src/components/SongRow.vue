<template>
    <div class="group flex items-center px-4 py-2 rounded-sm hover:bg-base-200 transition-colors cursor-pointer select-none border-b border-base-200/50"
        :class="{ 'bg-info/5': isActive }" @click="handleClick">

        <!-- Column 1: # -->
        <div
            class="w-12 text-center shrink-0 text-sm font-mono opacity-60 relative h-8 flex items-center justify-center">
            <span class="group-hover:hidden">{{ index + 1 }}</span>
            <button class="hidden group-hover:flex items-center justify-center w-full h-full text-info absolute inset-0"
                @click.stop="$emit('play', track)">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                        clip-rule="evenodd" />
                </svg>
            </button>
        </div>

        <!-- Column 2: Cover & Title/Artist (flex-1 matches header pl-14) -->
        <div class="flex-1 flex items-center min-w-0">
            <div v-if="showCover" class="w-10 h-10 rounded overflow-hidden shrink-0 shadow-sm relative mr-4">
                <img :src="coverSrc" class="w-full h-full object-cover" @error="handleImageError" loading="lazy"
                    decoding="async" />
                <div class="absolute inset-0 bg-black/10 group-hover:bg-transparent transition-colors"></div>
            </div>
            <div class="flex flex-col min-w-0" :class="{ 'pl-14': !showCover }">
                <span class="font-medium text-sm truncate" :class="{ 'text-info': isActive }">
                    {{ track.title || track.name || 'Sin título' }}
                </span>
                <span class="text-xs text-base-content/60 truncate group-hover:text-base-content/80 transition-colors">
                    {{ track.artist || 'Artista desconocido' }}
                </span>
            </div>
        </div>

        <!-- Column 3: Album (w-48) -->
        <div class="hidden md:block w-52 truncate text-left pr-4 shrink-0">
            <span class="text-xs text-base-content/50">
                {{ track.album || '' }}
            </span>
        </div>

        <!-- Column 4: Duration & Actions (w-20) -->
        <div class="w-28 flex items-center justify-end shrink-0 relative pe-1">
            <!-- Heart Button (Favorite) -->
            <button
                class="btn btn-ghost btn-xs btn-circle opacity-0 group-hover:opacity-100 transition-opacity absolute right-18"
                :class="{ 'opacity-100 text-error': isFavorite }" @click.stop="toggleFavorite">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" :fill="isFavorite ? 'currentColor' : 'none'"
                    viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
            </button>

            <!-- Duration -->
            <span
                class="text-xs font-mono text-base-content/60 transition-transform duration-200 group-hover:-translate-x-7">
                {{ formatDuration(track.duration) }}
            </span>

            <!-- Three Dots Menu -->
            <button
                class="btn btn-ghost btn-xs btn-circle opacity-0 group-hover:opacity-100 transition-opacity absolute right-0"
                @click.stop="openMenu">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z" />
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup>
import { computed } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useUIStore } from '../store/ui';

const props = defineProps({
    track: { type: Object, required: true },
    index: { type: Number, default: 0 },
    showCover: { type: Boolean, default: true },
    isActive: { type: Boolean, default: false },
    isFavorite: { type: Boolean, default: false },
    playlistId: { type: Number, default: null }
});

const emit = defineEmits(['play', 'toggleFavorite', 'playNext', 'addToQueue', 'showDetails']);
const uiStore = useUIStore();

const PLACEHOLDER = "https://placehold.co/120x120?text=No+Cover";

const coverSrc = computed(() => {
    return props.track.cover_path ? convertFileSrc(props.track.cover_path) : PLACEHOLDER;
});

function handleImageError(e) {
    if (e.target.src !== PLACEHOLDER) e.target.src = PLACEHOLDER;
}

function formatDuration(seconds) {
    if (!seconds || isNaN(seconds)) return '--:--';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
}

function toggleFavorite() {
    emit('toggleFavorite', props.track);
}

function handleClick(e) {
    // Prevent play if selecting text or clicking weird things, but generally play.
    emit('play', props.track);
}

function openMenu(e) {
    uiStore.showContextMenu(e, props.track, null, props.playlistId);
}
</script>
