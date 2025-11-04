import { defineStore } from "pinia"
import { ref } from "vue"

export const usePlayerStore = defineStore("player", () => {
    const currentTrack = ref(null)
    const isPlaying = ref(false)
    const volume = ref(1)

    function play(track) {
        currentTrack.value = track
        isPlaying.value = true
    }

    function pause() {
        isPlaying.value = false
    }

    return { currentTrack, isPlaying, volume, play, pause }
})
