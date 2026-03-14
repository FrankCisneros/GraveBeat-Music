<template>
    <div class="min-h-screen bg-[#212529] flex items-center justify-center p-4 font-pixel select-none">

        <div class="pixel-border bg-[#ced4da] p-6 max-w-md w-full shadow-[8px_8px_0_0_rgba(0,0,0,0.5)]">

            <div class="bg-[#adb5bd] border-4 border-black p-4 mb-6 shadow-inner relative overflow-hidden">
                <div class="w-full h-48 bg-[#000] flex items-center justify-center mb-4 border-2 border-black">
                    <div class="pixel-animation text-[#92cd41] text-center">
                        <p class="text-[10px] mb-2">NOW PLAYING</p>
                        <div class="flex gap-1 justify-center items-end h-12">
                            <div v-for="n in 8" :key="n" class="w-4 bg-[#92cd41] animate-bounce-pixel"
                                :style="{ animationDelay: `${n * 0.1}s` }"></div>
                        </div>
                    </div>
                </div>

                <h2 class="text-black text-sm uppercase truncate">Mega Man Theme</h2>
                <p class="text-[#495057] text-[10px] uppercase">Capcom Sound Team</p>
            </div>

            <div class="mb-6">
                <div class="h-4 w-full bg-[#495057] border-2 border-black p-0.5">
                    <div class="h-full bg-[#e76e55] w-[65%]" style="box-shadow: inset -2px 0 0 #8a2512"></div>
                </div>
                <div class="flex justify-between mt-1 text-[8px] text-black">
                    <span>01:45</span>
                    <span>03:20</span>
                </div>
            </div>

            <div class="grid grid-cols-3 gap-4">
                <button class="pixel-btn bg-[#f7d51d] active:translate-y-1">
                    <span class="block -mt-1">PREV</span>
                </button>
                <button @click="isPlaying = !isPlaying" class="pixel-btn bg-[#92cd41] active:translate-y-1">
                    <span class="block -mt-1">{{ isPlaying ? 'PAUSE' : 'PLAY' }}</span>
                </button>
                <button class="pixel-btn bg-[#f7d51d] active:translate-y-1">
                    <span class="block -mt-1">NEXT</span>
                </button>
            </div>

            <div class="mt-8 flex items-center gap-4">
                <span class="text-[8px] text-black">VOL</span>
                <input type="range" class="pixel-range w-full cursor-pointer" />
            </div>

        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const isPlaying = ref(false);
</script>

<style scoped>
/* Importación de fuente para el look retro */
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');
@reference "../style.css";


.font-pixel {
    font-family: 'Press Start 2P', cursive;
}

/* Efecto de borde escalonado Pixel Art */
.pixel-border {
    box-shadow:
        0 -4px 0 0 black,
        0 4px 0 0 black,
        -4px 0 0 0 black,
        4px 0 0 0 black;
}

/* Botón estilo RPG */
.pixel-btn {
    @apply py-3 border-none relative text-[10px] text-black;
    box-shadow:
        0 -4px 0 0 black inset,
        -4px 0 0 0 black inset,
        4px 0 0 0 rgba(255, 255, 255, 0.5) inset,
        0 4px 0 0 rgba(255, 255, 255, 0.5) inset,
        0 4px 0 0 black,
        0 -4px 0 0 black,
        -4px 0 0 0 black,
        4px 0 0 0 black;
}

/* Animación simplificada para el visualizador */
@keyframes bounce-pixel {

    0%,
    100% {
        height: 10%;
    }

    50% {
        height: 80%;
    }
}

.animate-bounce-pixel {
    animation: bounce-pixel 0.6s infinite ease-in-out;
}

/* Personalización del slider (Input Range) */
.pixel-range {
    -webkit-appearance: none;
    background: transparent;
}

.pixel-range::-webkit-slider-runnable-track {
    width: 100%;
    height: 8px;
    background: #495057;
    border: 2px solid black;
}

.pixel-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 16px;
    width: 8px;
    background: #e76e55;
    border: 2px solid black;
    margin-top: -6px;
}
</style>