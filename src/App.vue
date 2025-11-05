<template>
  <div class="flex flex-col h-screen" :data-theme="temaActual">
    <!-- Navbar superior -->
    <nav class="navbar bg-base-200 px-6 py-4 shadow-sm flex-shrink-0">
      <div class="flex-1">
        <a class="text-xl font-bold">🎵 MusicApp</a>
      </div>
      <div class="flex items-center space-x-4 ">
        
        <!-- Barra de búsqueda -->
        <div class="form-control">
          <div class="input-group">
            <input type="text" placeholder="Search songs, artists..." class="input input-bordered input-sm w-64 border-0 focus:outline-none justify-center" />
            <button class="btn btn-square btn-sm">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Perfil y notificaciones -->
        <div class="dropdown dropdown-end">
          <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
            <div class="w-8 rounded-full bg-blue-500 flex items-center justify-center">
              <span class="text-white text-sm font-bold">U</span>
            </div>
          </div>
          <ul tabindex="0" class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow">
            <li><a>Profile</a></li>
            <li><a>Settings</a></li>
            <li><a>Logout</a></li>
          </ul>
        </div>

        <!-- Selector de tema -->
        <select v-model="temaActual" class="select select-bordered select-sm w-32 focus:outline-none">
          <option disabled>Theme</option>
          <option v-for="t in temas" :key="t" :value="t">{{ t }}</option>
        </select>
        <button class="btn btn-outline btn-error" @click="closeApp">Close</button>
      </div>
    </nav>

    <!-- Contenido principal con sidebar - Área que hace scroll -->
    <div class="flex flex-1 min-h-0">
      <!-- Sidebar lateral con scroll -->
      <aside class="w-64 bg-base-100 border-r border-base-300 hidden md:flex flex-col">
        <div class="p-6 flex-1 overflow-y-auto">
          <!-- Navegación principal -->
          <div class="space-y-2 mb-8">
            <!-- Biblioteca -->
            <router-link class="flex items-center space-x-3 p-3 rounded-lg hover:bg-base-200 transition-colors" to="/library">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
              <span>Libreria</span>
            </router-link>

            <!-- Album -->
            <router-link class="flex items-center space-x-3 p-3 rounded-lg hover:bg-base-200 transition-colors" to="/files">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4M4 4l8 4 8-4M4 8l8 4 8-4m-8 4v8" />
              </svg>
              <span>Albums</span>
            </router-link>

            <router-link class="flex items-center space-x-3 p-3 rounded-lg hover:bg-base-200 transition-colors" to="/now-playing">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span>Now Playing</span>
            </router-link>
            
            <router-link class="flex items-center space-x-3 p-3 rounded-lg hover:bg-base-200 transition-colors" to="/settings">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <span>Settings</span>
            </router-link>
            
            <router-link class="flex items-center space-x-3 p-3 rounded-lg hover:bg-base-200 transition-colors" to="/about">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span>About</span>
            </router-link>
          </div>

          <!-- Playlists -->
          <div class="mt-8">
            <h2 class="text-lg font-semibold mb-4 text-gray-600">Your Playlists</h2>
            <ul class="space-y-2">
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-purple-500 to-pink-500 rounded flex items-center justify-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                    </svg>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Favorites</span>
                </a>
              </li>
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-cyan-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">CV</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Chill Vibes</span>
                </a>
              </li>
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-red-500 to-orange-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">WM</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Workout Mix</span>
                </a>
              </li>
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-green-500 to-emerald-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">TH</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Top Hits</span>
                </a>
              </li>
              <!-- Playlists adicionales para demostrar el scroll -->
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-yellow-500 to-orange-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">RM</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Road Trip</span>
                </a>
              </li>
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-indigo-500 to-purple-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">SM</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Study Mix</span>
                </a>
              </li>
              <li>
                <a href="#" class="flex items-center space-x-3 p-2 rounded-lg hover:bg-base-200 transition-colors group">
                  <div class="w-8 h-8 bg-gradient-to-br from-pink-500 to-rose-500 rounded flex items-center justify-center">
                    <span class="text-white text-xs">PM</span>
                  </div>
                  <span class="group-hover:text-blue-500 transition-colors">Party Mix</span>
                </a>
              </li>
            </ul>
          </div>

          <!-- Nueva playlist -->
          <button class="btn btn-ghost btn-sm w-full mt-6 justify-start space-x-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            <span>New Playlist</span>
          </button>
        </div>
      </aside>

      <!-- Área principal circular con scroll -->
      <main class="flex-1 min-h-0 p-4">
        <div class="rounded-[3rem] bg-gradient-to-br from-base-200 to-base-300 h-full p-8 shadow-xl overflow-y-auto">
          <router-view />
        </div>
      </main>
    </div>

    <!-- Player fijo en la parte inferior -->
    <footer class="bg-base-300 border-t border-base-200 flex-shrink-0">
      <PlayerControls />
    </footer>
  </div>
</template>

<script setup>
import { ref, watch } from "vue"
import PlayerControls from "./components/PlayerControls.vue"
import { getCurrentWindow } from '@tauri-apps/api/window';

const temas = ["light", "dark", "cupcake", "winter", "night", "emerald", "forest", "dracula"]
const temaActual = ref(localStorage.getItem("tema") || "cupcake")

watch(temaActual, (nuevo) => {
  document.documentElement.setAttribute("data-theme", nuevo)
  localStorage.setItem("tema", nuevo)
})

async function closeApp() {
  await getCurrentWindow().close();
}
</script>

<style scoped>
.router-link-active {
  background-color: hsl(var(--primary) / 1);
  color: hsl(var(--primary-content) / 1);
  font-weight: 500;
}

.rounded-\[3rem\] {
  border-radius: 1rem;
}

/* Asegurar que ocupe toda la pantalla */
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>