<template>
    <div class="p-4">
        <h1 class="text-2xl font-bold mb-6">🎶 Biblioteca local</h1>

        <div class="flex gap-2 mb-6">
            <input 
                v-model="dir" 
                class="input input-bordered flex-1 max-w-2xl"
                placeholder="Ruta de carpeta (ej: C:\Users\gungr\Music)" 
            />
            <button class="btn btn-primary" @click="startScan" :disabled="loading">
                {{ loading ? 'Escaneando...' : 'Escanear' }}
            </button>
        </div>

        <div v-if="error" class="alert alert-error mb-4">
            <span>{{ error }}</span>
        </div>

        <div v-if="tracks.length" class="mt-6">
            <div class="space-y-2">
                <div 
                    v-for="track in tracks" 
                    :key="track.path" 
                    class="flex items-center gap-4 p-4 bg-base-100 rounded-lg hover:bg-base-200 transition-colors"
                >
                    <!-- Carátula -->
                    <div class="flex-shrink-0">
                        <img 
                            :src="track.cover_data_url || 'https://placehold.co/80x80'" 
                            class="w-16 h-16 object-cover rounded"
                            alt="Carátula del álbum"
                        />
                    </div>

                    <!-- Información de la canción -->
                    <div class="flex-1 min-w-0">
                        <div class="font-semibold text-lg truncate">
                            {{ track.title || track.name || 'Sin título' }}
                        </div>
                        <div class="text-sm text-gray-500 truncate">
                            {{ track.artist || 'Artista desconocido' }}
                        </div>
                        <div class="text-xs text-gray-400 truncate">
                            {{ track.album || 'Álbum desconocido' }}
                        </div>
                    </div>

                    <!-- Duración -->
                    <div class="flex-shrink-0 w-20 text-right">
                        <span class="text-sm text-gray-500">
                            {{ formatDuration(track.duration) }}
                        </span>
                    </div>

                    <!-- Botón Play -->
                    <div class="flex-shrink-0">
                        <button 
                            class="btn btn-circle btn-primary btn-sm"
                            @click="playTrack(track)"
                            :disabled="!isPlayable(track)"
                            :title="isPlayable(track) ? 'Reproducir' : 'Archivo no reproducible'"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>

            <!-- Resumen -->
            <div class="mt-4 text-sm text-gray-500">
                Total: {{ tracks.length }} canción{{ tracks.length !== 1 ? 'es' : '' }}
            </div>
        </div>

        <!-- Estado vacío -->
        <div v-else-if="!loading" class="text-center py-12 text-gray-500">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto mb-4 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3" />
            </svg>
            <p>Ingresa una ruta y haz clic en "Escanear" para buscar música</p>
        </div>
    </div>
</template>

<script setup>
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"

const dir = ref("")
const tracks = ref([])
const loading = ref(false)
const error = ref("")

async function startScan() {
    if (!dir.value.trim()) {
        error.value = "Por favor, ingresa una ruta válida."
        return
    }

    loading.value = true
    error.value = ""
    tracks.value = []

    try {
        const result = await invoke("scan_folder", { path: dir.value })
        console.log("Archivos escaneados:", result)
        tracks.value = Array.isArray(result) ? result : []
    } catch (err) {
        console.error("Error al escanear carpeta:", err)
        error.value = String(err)
    } finally {
        loading.value = false
    }
}

function formatDuration(seconds) {
    if (!seconds || isNaN(seconds)) return '--:--'
    
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
}

function playTrack(track) {
    console.log('Reproduciendo:', track)
    // Aquí puedes implementar la lógica de reproducción
    // Por ejemplo:
    // audioPlayer.src = track.path
    // audioPlayer.play()
    
    // Mientras tanto, mostramos un mensaje en consola
    alert(`Reproduciendo: ${track.title || track.name}\nArtista: ${track.artist || 'Desconocido'}`)
}

function isPlayable(track) {
    // Verifica si el track tiene los datos mínimos para ser reproducible
    return track && track.path && (track.title || track.name)
}
</script>

<style scoped>
/* Estilos adicionales para mejor apariencia */
.truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.hover\:bg-base-200:hover {
    transition: background-color 0.2s ease-in-out;
}
</style>