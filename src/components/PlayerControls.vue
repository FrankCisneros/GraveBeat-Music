<template>
    <div class="flex justify-between items-center px-4 py-3 bg-base-200 border-t border-base-300">
        <!-- Sección izquierda: Info de la canción -->
        <div class="flex items-center space-x-4 w-1/4 min-w-0">
            <img class="w-14 h-14 rounded-md select-none pointer-events-none object-cover shadow-sm"
                :src="currentTrack?.cover_path ? coverSrc(currentTrack.cover_path) : PLACEHOLDER"
                :alt="currentTrack?.title || 'Track image'" draggable="false">
            <div class="flex flex-col select-none min-w-0">
                <div class="font-bold text-sm truncate">{{ currentTrack?.title || "No track" }}</div>
                <div class="text-xs text-gray-500 truncate">{{ currentTrack?.artist || "Unknown Artist" }}</div>
            </div>
            <button class="btn btn-ghost btn-xs text-gray-500 hover:text-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
            </button>
        </div>

        <!-- Sección central: Controles principales -->
        <div class="flex flex-col items-center space-y-2 w-2/4">
            <div class="flex justify-center items-center space-x-3">
                <button class="btn btn-ghost btn-sm hover:text-white hover:bg-transparent"
                    :class="player.shuffle ? 'text-primary' : 'text-gray-400'" @click="shuffle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                    </svg>
                </button>

                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent"
                    @click="previousTrack">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                    </svg>
                </button>

                <button class="btn btn-circle btn-sm bg-white text-black hover:bg-gray-200 border-0"
                    @click="togglePlay">
                    <svg v-if="!isPlaying" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 ml-0.5" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                            clip-rule="evenodd" />
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20"
                        fill="currentColor">
                        <path fill-rule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zM7 8a1 1 0 012 0v4a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v4a1 1 0 102 0V8a1 1 0 00-1-1z"
                            clip-rule="evenodd" />
                    </svg>
                </button>

                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white hover:bg-transparent"
                    @click="nextTrack">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </button>

                <button class="btn btn-ghost btn-sm hover:text-white hover:bg-transparent"
                    :class="player.repeat ? 'text-primary' : 'text-gray-400'" @click="repeat">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                    </svg>
                </button>
            </div>

            <div class="flex justify-center items-center w-full max-w-lg space-x-3">
                <span class="text-xs text-gray-400 select-none">{{ formatTime(currentTime) }}</span>
                <div class="relative w-full">
                    <input type="range" min="0" :max="duration || 0" v-model="sliderValue" class="seekbar text-primary"
                        @input="onSeekInput" @change="onSeekChange" />
                </div>
                <span class="text-xs text-gray-400 select-none">{{ formatTime(duration) }}</span>
            </div>
        </div>

        <!-- Sección derecha: Volumen y más opciones -->
        <div class="flex items-center justify-end space-x-3 w-1/4">
            <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
            </button>

            <!-- Equalizer Toggle -->
            <div class="relative">
                <button class="btn btn-ghost btn-sm hover:text-white" :class="showEQ ? 'text-primary' : 'text-gray-400'"
                    @click="showEQ = !showEQ">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
                    </svg>
                </button>

                <!-- Equalizer Popover -->
                <div v-show="showEQ"
                    class="absolute bottom-full right-0 mb-4 p-4 bg-base-300 rounded-lg shadow-xl z-50 w-[680px] border border-base-100">

                    <div class="flex justify-between items-center mb-4">
                        <h3 class="text-xs font-bold text-gray-400 uppercase tracking-widest">Equalizer</h3>
                        <select class="select select-bordered select-xs w-32" v-model="player.activePreset"
                            @change="applyPreset($event.target.value)">
                            <option value="Custom">Custom</option>
                            <option v-for="(val, name) in presets" :key="name" :value="name">{{ name }}</option>
                        </select>
                    </div>

                    <div class="flex justify-between h-50 space-x-0.10">
                        <div v-for="(freq, index) in frequencies" :key="index"
                            class="flex flex-col items-center flex-1 group">
                            <span class="mb-4 text-[10px] text-gray-500 font-mono invisible whitespace-nowrap"
                                :class="{ 'visible!': index % 2 === 0 }">{{ formatFreq(freq) }}</span>
                            <div class="relative h-28 w-full flex justify-center">
                                <input type="range" min="-12" max="12" step="0.5" v-model.number="eqLocalValues[index]"
                                    @input="updateEQ"
                                    class="eq-slider absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 -rotate-90 origin-center w-28 h-4">
                            </div>
                            <span class="mt-1 text-[10px] text-gray-500 font-mono invisible whitespace-nowrap"
                                :class="{ 'visible!': index % 2 !== 0 }">{{ formatFreq(freq) }}</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex items-center space-x-2">
                <button class="btn btn-ghost btn-sm text-gray-400 hover:text-white" @click="toggleMute">
                    <svg v-if="!isMuted && volume > 0.5" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none"
                        viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M15.536 8.464a5 5 0 010 7.072M12 6a9 9 0 010 12m-4.5-9.5L12 3v18l-4.5-4.5H4a1 1 0 01-1-1v-7a1 1 0 011-1h3.5z" />
                    </svg>
                    <svg v-else-if="!isMuted && volume > 0" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5"
                        fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 6a9 9 0 010 12m-4.5-9.5L12 3v18l-4.5-4.5H4a1 1 0 01-1-1v-7a1 1 0 011-1h3.5z" />
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                        stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"
                            clip-rule="evenodd" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2" />
                    </svg>
                </button>

                <input type="range" min="0" max="1" step="0.01" v-model="volume" class="seekbar w-24">
            </div>
            <!-- Song Details Toggle -->
            <button class="btn btn-ghost btn-sm hover:text-white"
                :class="isDetailsVisible ? 'text-primary' : 'text-gray-400'"
                @click="isDetailsVisible = !isDetailsVisible" title="Ver Detalles">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup>
import { usePlayerStore } from "../store/player"
import { storeToRefs } from "pinia"
import { ref, computed, watch, onMounted } from "vue"
import { convertFileSrc } from "@tauri-apps/api/core"
import { invoke } from "@tauri-apps/api/core"

const player = usePlayerStore()
const { isPlaying, currentTrack, volume, duration, currentTime, eqValues, isDetailsVisible } = storeToRefs(player)
const { frequencies, presets } = player

// Estados adicionales
const isMuted = ref(false)
const sliderValue = ref(0)
const isDragging = ref(false)
const showEQ = ref(false)
const eqLocalValues = ref([...eqValues.value])

// Sync local EQ with store EQ (when preset changes)
watch(eqValues, (newVal) => {
    eqLocalValues.value = [...newVal]
}, { deep: true })

function updateEQ() {
    player.setEQ(eqLocalValues.value)
}

function applyPreset(name) {
    player.applyPreset(name)
}

function formatFreq(freq) {
    return freq >= 1000 ? (freq / 1000).toFixed(1) + 'k' : freq
}

// Sync slider with store time
watch(currentTime, (newVal) => {
    if (!isDragging.value) {
        sliderValue.value = newVal
    }
})

// Mute toggle
const previousVolume = ref(1)
function toggleMute() {
    if (isMuted.value) {
        player.setVolume(previousVolume.value)
        isMuted.value = false
    } else {
        previousVolume.value = volume.value
        player.setVolume(0)
        isMuted.value = true
    }
}

// Función para formatear el tiempo
function formatTime(seconds) {
    if (!seconds) return "0:00"
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs < 10 ? '0' : ''}${secs}`
}

// Funciones de control
function togglePlay() {
    player.toggle()
}

function previousTrack() {
    player.prev()
}

function nextTrack() {
    player.next()
}

function shuffle() {
    player.shuffle = !player.shuffle
}

function repeat() {
    player.repeat = !player.repeat
}

function onSeekInput() {
    isDragging.value = true
}

function onSeekChange() {
    player.seek(sliderValue.value)
    isDragging.value = false

    if (!currentTrack.value) return
    invoke("update_presence", {
        title: currentTrack.value.title || "No track",
        artist: currentTrack.value.artist || "Unknown Artist",
        duration: Math.floor(duration.value || 0),
        currentTime: Math.floor(sliderValue.value || 0),
        playing: isPlaying.value
    })
}

const PLACEHOLDER = "https://placehold.co/100x100?text=No+Cover"

function coverSrc(path) {
    return path ? convertFileSrc(path) : PLACEHOLDER
}

watch(currentTrack, (track) => {
    if (!track) return

    invoke("update_presence", {
        title: track.title || "No track",
        artist: track.artist || "Unknown Artist",
        duration: Math.floor(duration.value || 0),
        currentTime: 0,
        playing: isPlaying.value
    })
})
</script>

<style scoped>
/* Main Seekbar & Volume Styles */
.seekbar {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 4px;
    background: #3f3f46;
    border-radius: 999px;
    cursor: pointer;
    overflow: hidden;
    /* For progress fill effect if desired, but tricky with plain CSS */
    /* @apply text-primary; moved to class */
}

.seekbar::-webkit-slider-runnable-track {
    height: 4px;
    background: #3f3f46;
}

.seekbar::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 4px;
    width: 4px;
    background: white;
    /* Thumb color */
    border-radius: 50%;
    cursor: pointer;
    box-shadow: -100vw 0 0 100vw white;
    /* Trick for progress fill */
}

.seekbar:hover::-webkit-slider-thumb {
    height: 10px;
    width: 10px;
    margin-top: -3px;
    /* Center it */
    box-shadow: -100vw 0 0 100vw currentColor;
}

/* EQ Slider Styles - Custom Styled to match user preference */
.eq-slider {
    -webkit-appearance: none;
    appearance: none;
    background: #3f3f46;
    /* Track background */
    border-radius: 999px;
    cursor: pointer;
    height: 4px;
}

.eq-slider::-webkit-slider-runnable-track {
    height: 4px;
    background: transparent;
}

.eq-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 12px;
    width: 12px;
    background-color: #4facfe;
    border-radius: 50%;
    margin-top: -4px;
    /* Center thumb on track */
    cursor: pointer;
    transition: transform 0.1s;
}

.eq-slider:hover::-webkit-slider-thumb {
    transform: scale(1.2);
    background: white;
}

/* Botones estilo GameBoy */
.pixel-btn-control {
    padding: 8px 12px;
    border: 4px solid black;
    box-shadow: inset -4px -4px 0 0 rgba(0, 0, 0, 0.2);
    font-size: 12px;
    cursor: pointer;
}

.pixel-btn-control:active {
    box-shadow: inset 4px 4px 0 0 rgba(0, 0, 0, 0.2);
    transform: translateY(2px);
}

/* La Seekbar ahora parece una barra de energía */
.pixel-seekbar {
    -webkit-appearance: none;
    width: 100%;
    height: 100%;
    background: #3f3f46;
    cursor: pointer;
}

.pixel-seekbar::-webkit-slider-runnable-track {
    height: 100%;
    background: #3f3f46;
}

.pixel-seekbar::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 100%;
    width: 12px;
    background: #e76e55;
    /* Color "Vida" */
    box-shadow: -100vw 0 0 100vw #92cd41;
    /* Color "Progreso" */
    border: 2px solid black;
}
</style>