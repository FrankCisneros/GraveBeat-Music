<template>
  <div class="h-full flex flex-col p-4 overflow-hidden">
    <div class="flex items-center justify-between mb-6 shrink-0">
      <h1 class="text-3xl font-bold bg-clip-text text-transparent bg-linear-to-r from-primary to-secondary">
        🎤 Artistas
      </h1>
      <div class="text-sm opacity-50">{{ artists.length }} encontrados</div>
    </div>

    <div v-if="loading && !artists.length" class="flex-1 flex justify-center items-center">
      <span class="loading loading-dots loading-lg text-primary"></span>
    </div>

    <div v-else-if="artists.length === 0" class="flex-1 flex flex-col items-center justify-center opacity-50">
      <span class="text-6xl mb-4">🎸</span>
      <p>No hay artistas en tu biblioteca.</p>
    </div>

    <div v-else class="flex-1 overflow-y-auto pr-2 custom-scroll">
      <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6 p-1">
        <div v-for="artist in artists" :key="artist.name"
          class="card bg-base-100 shadow-xl hover:shadow-2xl hover:scale-105 transition-all duration-300 cursor-pointer border border-base-200"
          @click="goToArtist(artist)">
          <figure class="px-4 pt-6 relative group">
            <div class="avatar">
              <div
                class="w-32 h-32 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2 overflow-hidden shadow-lg">
                <img :src="getArtistImage(artist)" alt="Artist"
                  class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
                  @error="handleImageError" />
              </div>
            </div>
          </figure>
          <div class="card-body items-center text-center p-4">
            <h2 class="card-title text-base line-clamp-1" :title="artist.name">
              {{ artist.name || 'Desconocido' }}
            </h2>
            <div class="badge badge-ghost badge-sm gap-1">
              🎵 {{ artist.song_count }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, computed, ref, watch } from 'vue';
import { useArtistsStore } from '../store/artists';
import { useRouter } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';

import { useSettingsStore } from '../store/settings';

const store = useArtistsStore();
const settingsStore = useSettingsStore();

const router = useRouter();

const artists = computed(() => store.artists);
const loading = computed(() => store.loading);

watch(() => settingsStore.activeProfileId, () => {
  store.fetchArtists();
});

const DEFAULT_IMAGE = "https://ui-avatars.com/api/?background=random&name=";

onMounted(async () => {
  // Force refresh or just load if empty?
  // Better to always fetch to get latest counts/images
  await store.fetchArtists();

  // Background fetch images for those missing them
  artists.value.forEach(artist => {
    if (!artist.image_path) {
      store.ensureArtistImage(artist);
    }
  });

  // If "Unknown" artist exists, maybe skip image fetch?
});

const goToArtist = (artist) => {
  router.push({ name: 'ArtistDetails', params: { name: artist.name } });
};

const getArtistImage = (artist) => {
  if (artist.image_path) {
    if (artist.image_path.startsWith('http')) return artist.image_path;
    return convertFileSrc(artist.image_path);
  }
  return DEFAULT_IMAGE + encodeURIComponent(artist.name);
};

const handleImageError = (e) => {
  // e.target.src = "https://placehold.co/200x200?text=?";
};
</script>

<style scoped>
.custom-scroll::-webkit-scrollbar {
  width: 8px;
}

.custom-scroll::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scroll::-webkit-scrollbar-thumb {
  background-color: var(--fallback-bc, oklch(var(--bc)/0.2));
  border-radius: 4px;
}
</style>
