<template>
    <aside
        class="bg-base-100 border-r border-base-300 hidden md:flex flex-col h-full transition-all duration-300 ease-in-out"
        :class="isCollapsed ? 'w-20' : 'w-64'">
        <!-- Collapse Toggle Area -->
        <button @click="isCollapsed = !isCollapsed"
            class="h-16 flex items-center shrink-0 border-b border-base-200 hover:bg-base-200 transition-colors w-full focus:outline-none cursor-pointer"
            :class="isCollapsed ? 'justify-center text-gray-400 hover:text-primary' : 'justify-start px-6 gap-4 text-gray-400 hover:text-primary'">

            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 shrink-0" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"
                    v-if="isCollapsed" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7"
                    v-else />
            </svg>
            <span v-if="!isCollapsed" class="font-bold text-xs uppercase tracking-widest">Tu biblioteca</span>
        </button>

        <!-- Scrollable Content -->
        <div class="flex-1 overflow-y-auto overflow-x-hidden py-4 custom-scroll">

            <!-- Discover Section -->
            <div class="mb-6">
                <h3 v-if="!isCollapsed"
                    class="px-6 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 transition-opacity duration-200">
                    Descubrir</h3>
                <div class="space-y-1">
                    <router-link to="/library" class="sidebar-link"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                        title="Inicio">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                        </svg>
                        <span v-if="!isCollapsed" class="truncate">Inicio</span>
                    </router-link>

                    <router-link to="/album" class="sidebar-link"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                        title="Álbumes">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                        </svg>
                        <span v-if="!isCollapsed" class="truncate">Álbumes</span>
                    </router-link>

                    <router-link to="/artists" class="sidebar-link"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                        title="Artistas">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                        </svg>
                        <span v-if="!isCollapsed" class="truncate">Artistas</span>
                    </router-link>
                </div>
            </div>

            <!-- Library Section -->
            <div class="mb-6">
                <h3 v-if="!isCollapsed"
                    class="px-6 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 transition-opacity duration-200">
                    Biblioteca</h3>
                <div class="space-y-1">
                    <router-link to="/favorites" class="sidebar-link"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                        title="Favoritos">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                        </svg>
                        <span v-if="!isCollapsed" class="truncate">Favoritos</span>
                    </router-link>

                    <router-link to="/recent" class="sidebar-link"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                        title="Reciente">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span v-if="!isCollapsed" class="truncate">Reciente</span>
                    </router-link>
                </div>
            </div>

            <!-- Playlists Section -->
            <div class="mb-6">
                <h3 v-if="!isCollapsed"
                    class="px-6 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 sticky top-0 bg-base-100 py-1 z-10">
                    Playlists</h3>
                <div class="space-y-1">
                    <button class="sidebar-link w-full text-left"
                        :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" @click="createPlaylist"
                        title="Nueva Playlist">
                        <div class="p-1 rounded bg-base-300 group-hover:bg-base-content group-hover:text-base-100 transition-colors"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M12 4v16m8-8H4" />
                            </svg>
                        </div>
                        <span v-if="!isCollapsed" class="truncate">Nueva Playlist</span>
                    </button>

                    <router-link v-for="playlist in playlists" :key="playlist.id" :to="`/playlist/${playlist.id}`"
                        class="sidebar-link group" :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }"
                        active-class="active-link" :title="playlist.name">
                        <div class="p-1 rounded bg-base-300 group-hover:bg-base-content group-hover:text-base-100 transition-colors opacity-70"
                            :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                                stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
                            </svg>
                        </div>
                        <span v-if="!isCollapsed" class="truncate">{{ playlist.name }}</span>
                    </router-link>
                </div>
            </div>
        </div>

        <!-- Settings Link -->
        <div class="py-4 border-t border-base-200">
            <router-link to="/settings" class="sidebar-link"
                :class="{ 'justify-center px-0': isCollapsed, 'pl-6': !isCollapsed }" active-class="active-link"
                title="Configuración">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                    :class="{ 'mr-0': isCollapsed, 'mr-3': !isCollapsed }" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                <span v-if="!isCollapsed">Configuración</span>
            </router-link>
        </div>
    </aside>

    <!-- Modal for Creating Playlist -->
    <dialog id="create_playlist_modal" class="modal">
        <div class="modal-box bg-base-200">
            <h3 class="font-bold text-lg mb-4">Nueva Playlist</h3>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Nombre de la Playlist</span>
                </label>
                <input type="text" v-model="newPlaylistName" placeholder="Escribe el nombre aquí"
                    class="input input-bordered w-full" @keyup.enter="submitCreatePlaylist" />
            </div>
            <div class="modal-action">
                <form method="dialog">
                    <button class="btn mr-2" @click="newPlaylistName = ''">Cancelar</button>
                    <button class="btn btn-primary" @click.prevent="submitCreatePlaylist">Crear</button>
                </form>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button @click="newPlaylistName = ''">close</button>
        </form>
    </dialog>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../store/settings'

const isCollapsed = ref(false)
const playlists = ref([])
const settingsStore = useSettingsStore()
const newPlaylistName = ref('')

async function loadPlaylists() {
    try {
        playlists.value = await invoke("get_playlists", {
            profileId: settingsStore.activeProfileId
        })
    } catch (err) {
        console.error("Error cargando playlists:", err)
    }
}

function createPlaylist() {
    // Open the daisyUI modal
    document.getElementById('create_playlist_modal').showModal()
}

async function submitCreatePlaylist() {
    const name = newPlaylistName.value.trim()
    if (!name) return

    try {
        await invoke("create_playlist", {
            name: name,
            description: "",
            profileId: settingsStore.activeProfileId
        })
        newPlaylistName.value = ''
        document.getElementById('create_playlist_modal').close()
        await loadPlaylists()
    } catch (error) {
        console.error("Error creando playlist:", error)
        alert("Hubo un error al crear la playlist")
    }
}

// Ensure playlists are refetched when the profile changes or loaded first time
onMounted(() => {
    loadPlaylists()
    window.addEventListener('playlists-updated', loadPlaylists)
})

onUnmounted(() => {
    window.removeEventListener('playlists-updated', loadPlaylists)
})

watch(() => settingsStore.activeProfileId, () => {
    loadPlaylists()
})
</script>

<style scoped>
.sidebar-link {
    display: flex;
    align-items: center;
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    line-height: 1.25rem;
    font-weight: 500;
    color: var(--color-base-content);
    border-top-right-radius: 0.375rem;
    border-bottom-right-radius: 0.375rem;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    border-left-width: 4px;
    border-color: transparent;
    cursor: pointer;
    min-height: 2.5rem;
    /* Ensure consistent height */
}

.sidebar-link:hover {
    color: var(--color-info);
    background-color: var(--color-base-100);
}

.active-link {
    color: var(--color-info);
    background-color: var(--color-base-100);
    border-color: var(--color-info);
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
