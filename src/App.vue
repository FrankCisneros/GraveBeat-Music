<template>
  <div class="flex flex-col h-screen overflow-hidden" :data-theme="temaActual">

    <NavigationBar v-model="temaActual" :temas="temas" />

    <div class="flex flex-1 min-h-0 relative">
      <Sidebar />

      <main class="flex-1 min-h-0 transition-all duration-300" :class="{ 'mr-80': isDetailsVisible }">
        <div class="rounded-4x1 p-3 h-full shadow-xl overflow-y-auto bg-linear-to-br from-base-100 to-base-300">
          <router-view />
        </div>
      </main>

      <SongDetailsSidebar :visible="isDetailsVisible" :track="currentTrack" @close="isDetailsVisible = false" />
    </div>

    <footer class="bg-base-300 border-t border-base-200 shrink-0 z-50">
      <PlayerControls />
    </footer>

    <ContextMenu />
  </div>
</template>

<script setup>
import { computed, watch, onMounted } from "vue";
import NavigationBar from "./components/NavigationBar.vue";
import PlayerControls from "./components/PlayerControls.vue";
import Sidebar from "./components/Sidebar.vue";
import SongDetailsSidebar from "./components/SongDetailsSidebar.vue";
import ContextMenu from "./components/ContextMenu.vue";
import { useSettingsStore } from "./store/settings";
import { usePlayerStore } from "./store/player";
import { storeToRefs } from "pinia";
import { getCurrentWindow } from '@tauri-apps/api/window';

const settingsStore = useSettingsStore()
const playerStore = usePlayerStore()

const { isDetailsVisible, currentTrack } = storeToRefs(playerStore)

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
})

watch(temaActual, (nuevo) => {
  document.documentElement.setAttribute("data-theme", nuevo);
  settingsStore.setTheme(nuevo)
});
</script>