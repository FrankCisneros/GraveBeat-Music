<template>
  <div class="flex flex-col h-screen overflow-hidden" :data-theme="temaActual">

    <NavigationBar v-model="temaActual" :temas="temas" />

    <div class="flex flex-1 min-h-0 relative">
      <Sidebar />

      <main class="flex-1 min-h-0 transition-all duration-300" :class="{ 'mr-80': isDetailsVisible }">
        <div class="rounded-4x1 p-1 h-full shadow-xl overflow-y-auto bg-linear-to-br from-base-100 to-base-300">
          <router-view />
        </div>
      </main>

      <SongDetailsSidebar :visible="isDetailsVisible" :track="currentTrack" @close="isDetailsVisible = false" />
    </div>

    <footer class="bg-linear-to-br from-base-100 to-base-300 border-t border-base-200 shrink-0 z-50">
      <PlayerControls />
    </footer>

    <ContextMenu />

    <!-- Global Scanning Modal -->
    <dialog class="modal" :class="{ 'modal-open': isScanning }">
      <div class="modal-box text-center">
        <h3 class="font-bold text-lg mb-2">Escaneando Biblioteca</h3>
        <p class="text-sm opacity-70 truncate mb-4">{{ scanMessage }}</p>

        <div class="w-full bg-base-200 rounded-full h-4 mb-2 overflow-hidden border border-base-300">
          <div class="bg-primary h-full transition-all duration-300" :style="{ width: percent + '%' }"></div>
        </div>

        <p class="font-mono text-sm">{{ scanProgress }} / {{ scanTotal }} canciones</p>
      </div>
    </dialog>

    <!-- Global Playlist Modal -->
    <dialog class="modal modal-bottom sm:modal-middle" :class="{ 'modal-open': uiStore.playlistModal.visible }">
      <div class="modal-box relative">
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          @click="uiStore.hidePlaylistModal()">✕</button>
        <h3 class="font-bold text-lg mb-4">Agregar a Playlist</h3>
        <p class="text-sm mb-4 opacity-80">Selecciona o crea una playlist para agregar <br> <span
            class="text-primary font-bold">{{ uiStore.playlistModal.track?.title || uiStore.playlistModal.track?.name
            }}</span></p>

        <div v-if="playlists.length === 0" class="text-center py-4 opacity-50 text-sm">
          No tienes playlists.
        </div>

        <div class="max-h-60 overflow-y-auto mb-4 custom-scroll">
          <button v-for="pl in playlists" :key="pl.id"
            class="w-full text-left px-4 py-3 hover:bg-base-200 text-sm transition-colors border-b border-base-200 last:border-0 rounded-lg"
            @click="addToPlaylist(pl.id)">
            <div class="font-semibold">{{ pl.name }}</div>
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop" @click="uiStore.hidePlaylistModal()">
        <button>Cerrar</button>
      </form>
    </dialog>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from "vue";
import NavigationBar from "./components/NavigationBar.vue";
import PlayerControls from "./components/PlayerControls.vue";
import Sidebar from "./components/Sidebar.vue";
import SongDetailsSidebar from "./components/SongDetailsSidebar.vue";
import ContextMenu from "./components/ContextMenu.vue";
import { useSettingsStore } from "./store/settings";
import { usePlayerStore } from "./store/player";
import { useUIStore } from "./store/ui";
import { storeToRefs } from "pinia";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/core";
import { register } from '@tauri-apps/plugin-global-shortcut';

const settingsStore = useSettingsStore()
const playerStore = usePlayerStore()
const uiStore = useUIStore()

const { isDetailsVisible, currentTrack } = storeToRefs(playerStore)
const { isScanning, scanProgress, scanTotal, scanMessage } = storeToRefs(settingsStore)

const percent = computed(() => {
  if (scanTotal.value === 0) return 0;
  return Math.min(100, (scanProgress.value / scanTotal.value) * 100);
})

const playlists = ref([])

watch(() => uiStore.playlistModal.visible, async (visible) => {
  if (visible) {
    try {
      playlists.value = await invoke("get_playlists", { profileId: settingsStore.activeProfileId })
    } catch (e) {
      console.error(e)
    }
  }
})

const addToPlaylist = async (playlistId) => {
  if (!uiStore.playlistModal.track) return;
  try {
    await invoke("add_song_to_playlist", {
      playlistId,
      songId: uiStore.playlistModal.track.id
    });
    uiStore.hidePlaylistModal();
  } catch (e) {
    alert("Ya está agregada o hubo un error.");
  }
}

const temaActual = computed({
  get: () => settingsStore.theme,
  set: (value) => settingsStore.setTheme(value)
})

const temas = settingsStore.availableThemes

onMounted(async () => {
  // Set initial theme
  document.documentElement.setAttribute("data-theme", settingsStore.theme)
  // Show window after content is ready
  await getCurrentWindow().show();

  // Escaneo en inicio si la carpeta fue modificada
  try {
    const lastScanStr = localStorage.getItem(`lastScan_${settingsStore.activeProfileId}`) || "0";
    const lastScan = parseInt(lastScanStr);

    await settingsStore.loadFolders();
    let needsScan = false;
    for (const folder of settingsStore.folders) {
      const isModified = await invoke("is_folder_modified", { path: folder, lastScan });
      if (isModified) {
        needsScan = true;
        break;
      }
    }

    if (needsScan) {
      await settingsStore.performScan();
    }
  } catch (e) {
    console.error("Error al comprobar modificaciones en las carpetas:", e);
  }

  // Teclas multimedia globales
  try {
    await register('MediaPlayPause', (event) => {
      if (event.state === 'Pressed') playerStore.toggle();
    });
    await register('MediaTrackNext', (event) => {
      if (event.state === 'Pressed') playerStore.next();
    });
    await register('MediaTrackPrevious', (event) => {
      if (event.state === 'Pressed') playerStore.prev();
    });
  } catch (err) {
    console.error("Error al registrar teclas multimedia locales globales:", err);
  }

  // Tecla espaciadora para reproducir/pausar (local en la ventana)
  window.addEventListener('keydown', (e) => {
    // Ignorar si el usuario está escribiendo en un input
    const tag = document.activeElement?.tagName?.toLowerCase();
    if (tag === 'input' || tag === 'textarea') return;

    if (e.code === 'Space') {
      e.preventDefault();
      playerStore.toggle();
    }
  });
})

watch(temaActual, (nuevo) => {
  document.documentElement.setAttribute("data-theme", nuevo);
  settingsStore.setTheme(nuevo)
});
</script>

<style scoped>
.custom-scroll::-webkit-scrollbar {
  width: 4px;
}

.custom-scroll::-webkit-scrollbar-thumb {
  background: #52525b;
  border-radius: 4px;
}
</style>