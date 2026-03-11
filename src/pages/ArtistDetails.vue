<template>
  <div class="h-full flex flex-col p-4 bg-base-100/50">
    <!-- Header -->
    <div class="flex items-center gap-6 mb-8 shrink-0">
      <button @click="goBack" class="btn btn-circle btn-ghost">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </button>

      <div class="avatar">
        <div class="w-32 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2 shadow-2xl">
          <img :src="artistImage" />
        </div>
      </div>

      <div>
        <h1 class="text-4xl font-bold mb-2">{{ artistName }}</h1>
        <div class="flex gap-4 text-sm opacity-70">
          <span>{{ songs.length }} Canciones</span>
          <!-- <span>{{ totalDuration }}</span> -->
        </div>
      </div>
    </div>

    <!-- Songs List -->
    <div class="flex-1 overflow-y-auto custom-scroll pr-2">
      <div v-if="loading" class="flex justify-center p-10">
        <span class="loading loading-spinner loading-lg"></span>
      </div>

      <div v-else class="bg-base-100/50 rounded-xl border border-base-200 shadow-inner overflow-hidden">
        <!-- Header -->
        <div
          class="flex items-center px-4 py-2 text-xs font-semibold text-base-content/50 uppercase tracking-wider border-b border-base-200">
          <div class="w-8 text-center shrink-0">#</div>
          <div class="flex-1 pl-14">Título</div>
          <div class="hidden md:block w-48 text-left pr-4">Álbum</div>
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
import { onMounted, computed, ref, watch } from 'vue';
import { useArtistsStore } from '../store/artists';
import { usePlayerStore } from '../store/player';
import { useRouter, useRoute } from 'vue-router';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import SongRow from '../components/SongRow.vue';
import { useSettingsStore } from '../store/settings';

const route = useRoute();
const router = useRouter();
const artistStore = useArtistsStore();
const playerStore = usePlayerStore();

const artistName = ref('');
const songs = computed(() => artistStore.currentArtistSongs);
const loading = computed(() => artistStore.loading);

const artistImage = computed(() => {
  const artist = artistStore.artists.find(a => a.name === artistName.value);
  if (artist && artist.image_path) {
    if (artist.image_path.startsWith('http')) return artist.image_path;
    return convertFileSrc(artist.image_path);
  }
  return `https://ui-avatars.com/api/?background=random&name=${encodeURIComponent(artistName.value)}`;
});

const loadData = async () => {
  artistName.value = route.params.name;
  if (artistName.value) {
    await artistStore.fetchArtistSongs(artistName.value);
  }
};

onMounted(() => {
  loadData();
});

watch(() => route.params.name, () => {
  loadData();
});

const goBack = () => {
  router.back();
};

const playSong = (song) => {
  playerStore.play(song);
};

const toggleFavorite = async (track) => {
  const settings = useSettingsStore();
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
  artistStore.artists = [...artistStore.artists] // trigger reactivity just in case
};

const getCover = (path) => {
  return path ? convertFileSrc(path) : "https://placehold.co/40x40?text=?";
};

const formatDuration = (seconds) => {
  if (!seconds) return '--:--';
  const min = Math.floor(seconds / 60);
  const sec = seconds % 60;
  return `${min}:${sec.toString().padStart(2, '0')}`;
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
