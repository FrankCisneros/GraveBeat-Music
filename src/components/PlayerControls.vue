<template>
    <div class="flex justify-between items-center px-4 py-3 bg-base-200 border-t border-base-300">
        <!-- Sección izquierda: Info de la canción -->
        <div class="flex items-center space-x-4 w-1/4 min-w-0">
            <img 
                class="w-14 h-14 rounded-md select-none pointer-events-none object-cover shadow-sm" 
                :src="currentTrack?.image || 'https://placehold.co/1000x1000'" 
                :alt="currentTrack?.title || 'Track image'"
                draggable="false"
            >
            <div class="flex flex-col select-none min-w-0">
                <div class="font-bold text-sm truncate">{{ currentTrack?.title || "No track" }}</div>
                <div class="text-xs text-gray-500 truncate">{{ currentTrack?.artist || "Unknown Artist" }}</div>
            </div>
            <button class="btn btn-ghost btn-xs text-gray-500 hover:text-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
            </button>
        </div>

        <!-- Sección central: Controles principales -->
        <div class="flex flex-col items-center space-y-2 w-2/4">
            <div class="flex justify-center items-center space-x-3">
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent" @click="shuffle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                    </svg>
                </button>
                
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent" @click="previousTrack">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                </button>
                
                <button 
                    class="btn btn-circle btn-sm bg-white text-black hover:bg-gray-200 border-0" 
                    @click="togglePlay"
                >
                    <svg v-if="!isPlaying" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 ml-0.5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z" clip-rule="evenodd" />
                    </svg>
                </button>
                
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent" @click="nextTrack">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </button>
                
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent" @click="repeat">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                    </svg>
                </button>
            </div>

            <div class="flex justify-center items-center w-full max-w-lg space-x-3">
                <span class="text-xs text-gray-400 select-none">{{ formatTime(currentTime) }}</span>
                <div class="relative w-full">
                    <input 
                        type="range" 
                        min="0" 
                        :max="duration" 
                        v-model="currentTime" 
                        class="range range-xs w-full bg-gray-600 cursor-pointer" 
                        @change="seek"
                    >
                    <div 
                        class="absolute top-1/2 left-0 h-1 bg-white rounded-full -translate-y-1/2" 
                        :style="{ width: progressPercentage + '%' }"
                    ></div>
                </div>
                <span class="text-xs text-gray-400 select-none">{{ formatTime(duration) }}</span>
            </div>
        </div>

        <!-- Sección derecha: Volumen y más opciones -->
        <div class="flex items-center justify-end space-x-3 w-1/4">
            <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
            </button>
            
            <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                </svg>
            </button>
            
            <div class="flex items-center space-x-2">
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white" @click="toggleMute">
                    <svg v-if="!isMuted && volume > 0.5" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.536 8.464a5 5 0 010 7.072M12 6a9 9 0 010 12m-4.5-9.5L12 3v18l-4.5-4.5H4a1 1 0 01-1-1v-7a1 1 0 011-1h3.5z" />
                    </svg>
                    <svg v-else-if="!isMuted && volume > 0" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6a9 9 0 010 12m-4.5-9.5L12 3v18l-4.5-4.5H4a1 1 0 01-1-1v-7a1 1 0 011-1h3.5z" />
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" clip-rule="evenodd" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
                    </svg>
                </button>
                
                <input 
                    type="range" 
                    min="0" 
                    max="1" 
                    step="0.01" 
                    v-model="volume" 
                    class="range range-xs w-20 bg-gray-600 cursor-pointer"
                >
            </div>
        </div>
    </div>
</template>

<script setup>
import { usePlayerStore } from "../store/player"
import { storeToRefs } from "pinia"
import { ref, computed, watch } from "vue"

const player = usePlayerStore()
const { isPlaying, currentTrack, volume } = storeToRefs(player)

// Estados adicionales para mejorar la funcionalidad
const currentTime = ref(0)
const duration = ref(180) // 3 minutos en segundos
const isMuted = ref(false)

// Computed para el progreso
const progressPercentage = computed(() => {
    return (currentTime.value / duration.value) * 100
})

// Función para formatear el tiempo
function formatTime(seconds) {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs < 10 ? '0' : ''}${secs}`
}

// Funciones de control
function togglePlay() {
    player.isPlaying ? player.pause() : player.play({ 
        title: "Demo Song", 
        artist: "Demo Artist",
        image: "https://placehold.co/1000x1000"
    })
}

function toggleMute() {
    isMuted.value = !isMuted.value
    // Aquí implementarías la lógica para silenciar el audio
}

function previousTrack() {
    // Lógica para canción anterior
    console.log("Previous track")
}

function nextTrack() {
    // Lógica para siguiente canción
    console.log("Next track")
}

function shuffle() {
    // Lógica para modo aleatorio
    console.log("Shuffle toggle")
}

function repeat() {
    // Lógica para modo repetición
    console.log("Repeat toggle")
}

function seek() {
    // Lógica para buscar en la canción
    console.log("Seek to:", currentTime.value)
}

// Simulación de progreso de reproducción
watch(isPlaying, (playing) => {
    if (playing) {
        // En una implementación real, esto se controlaría con el audio real
        const interval = setInterval(() => {
            if (currentTime.value < duration.value) {
                currentTime.value += 1
            } else {
                clearInterval(interval)
                isPlaying.value = false
            }
        }, 1000)
    }
})
</script>

<style scoped>
.range-xs {
    height: 4px;
}

.range-xs::-webkit-slider-thumb {
    height: 12px;
    width: 12px;
    opacity: 0;
    transition: opacity 0.2s;
}

.range-xs:hover::-webkit-slider-thumb {
    opacity: 1;
}

.range-xs::-moz-range-thumb {
    height: 12px;
    width: 12px;
    opacity: 0;
    transition: opacity 0.2s;
}

.range-xs:hover::-moz-range-thumb {
    opacity: 1;
}
</style>