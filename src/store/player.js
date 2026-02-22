import { defineStore } from "pinia"
import { convertFileSrc } from "@tauri-apps/api/core"
import { ref, watch } from "vue"
import { Howl, Howler } from "howler"

export const usePlayerStore = defineStore("player", () => {
    const savedPlaylist = localStorage.getItem("player_playlist")
    const playlist = ref(savedPlaylist ? JSON.parse(savedPlaylist) : [])

    const savedIndex = localStorage.getItem("player_currentIndex")
    const currentIndex = ref(savedIndex ? parseInt(savedIndex) : 0)

    const currentTrack = ref(playlist.value.length > 0 ? playlist.value[currentIndex.value] : null)
    const isPlaying = ref(false)
    const repeat = ref(false)
    const shuffle = ref(false)
    const duration = ref(0)
    const currentTime = ref(0)
    const isDetailsVisible = ref(false)

    // Persistence defaults
    const savedVolume = localStorage.getItem("player_volume")
    const volume = ref(savedVolume ? parseFloat(savedVolume) : 1)

    // Safety limit to prevent distortion from EQ gain
    const MAX_VOLUME_LIMIT = 0.5
    Howler.volume(volume.value * MAX_VOLUME_LIMIT)

    // EQ Bands (Standard 20-band EQ)
    const frequencies = [31, 43, 63, 87, 125, 175, 250, 350, 500, 700, 1000, 1400, 2000, 2800, 4000, 5600, 8000, 11200, 16000, 22000]
    const filters = [] // AudioNodes

    // Load saved EQ or default
    const savedEQ = localStorage.getItem("player_eq")
    const parsedEQ = savedEQ ? JSON.parse(savedEQ) : Array(20).fill(0)

    const eqValues = ref(parsedEQ) // Array of gains (-12 to 12)

    const savedPreset = localStorage.getItem("player_eq_preset") || "Flat"
    const activePreset = ref(savedPreset)

    // Internal state
    let sound = null
    let seekInterval = null
    let eqInitialized = false

    // Presets
    const presets = {
        "Flat": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        "Pop": [-1.5, 0.75, 3, 3.75, 4.5, 4.5, 4.5, 3.75, 3, 0.75, -1.5, -2.25, -3, -3, -3, -2.25, -1.5, -1.5, -1.5, -1.5],
        "Rock": [4.5, 3.75, 3, 0, -3, -3.75, -4.5, -3, -1.5, 0.75, 3, 3.75, 4.5, 5.25, 6, 6, 6, 6, 6, 6],
        "Jazz": [3, 3, 3, 0.75, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -0.75, 0, 0.75, 1.5, 2.25, 3, 3.75, 4.5, 4.5],
        "Classical": [3, 3, 3, 2.25, 1.5, 0.75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.75, 1.5, 2.25, 3, 3],
        "Bass": [6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        "Party": [4.5, 4.5, 4.5, 2.25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2.25, 4.5, 4.5, 4.5, 4.5],
        "Acoustic": [3, 3, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0.5, 1, 1.5, 2, 2.5, 3, 2.5, 2, 2],
        "Dance": [4, 5, 6, 4, 2, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 4.5, 5, 4.5, 4, 4],
        "Electronic": [4, 4, 3, 1, -1, -2, -2, -1, 0, 1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4],
        "Hip-Hop": [5, 6, 5, 3, 1, 0, -1, -1, 0, 1, 2, 3, 4, 4, 3, 2, 1, 0, 0, 0],
        "Lounge": [-2, -1, 0, 1, 2, 3, 4, 3, 2, 1, 0, 0, 0, 0, 0, -1, -2, -2, -2, -2],
        "Piano": [0, 0, 1, 2, 3, 4, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 3, 2, 1, 0],
        "R&B": [3, 4, 5, 4, 2, 0, -1, -1, -2, -1, 0, 1, 2, 3, 4, 4, 3, 2, 1, 1],
        "Treble Booster": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 6, 6],
        "Vocal Booster": [-2, -2, -1, -1, 0, 1, 2, 3, 4, 5, 4, 2, 1, 0, 0, -1, -1, -2, -2, -2]
    }

    function initEQ() {
        if (eqInitialized || !Howler.ctx) return

        try {
            const ctx = Howler.ctx

            // Create filters
            filters.length = 0 // Clear existing

            // Disconnect master gain first
            if (Howler.masterGain) {
                Howler.masterGain.disconnect()
            }

            let previousNode = Howler.masterGain

            frequencies.forEach((freq, index) => {
                const filter = ctx.createBiquadFilter()

                // First band: Lowshelf, Last band: Highshelf, Others: Peaking
                if (index === 0) {
                    filter.type = "lowshelf"
                } else if (index === frequencies.length - 1) {
                    filter.type = "highshelf"
                } else {
                    filter.type = "peaking"
                }

                filter.frequency.value = freq
                filter.gain.value = eqValues.value[index]

                // Chain nodes
                if (previousNode) {
                    previousNode.connect(filter)
                }
                previousNode = filter
                filters.push(filter)
            })

            // Connect last filter to destination
            if (previousNode) {
                previousNode.connect(ctx.destination)
            }

            eqInitialized = true
        } catch (e) {
            console.warn("Failed to initialize EQ:", e)
        }
    }

    function setEQ(values) {
        if (!Array.isArray(values) || values.length !== 20) return

        eqValues.value = [...values]

        // Apply to audio nodes if initialized
        if (eqInitialized && filters.length === 20) {
            filters.forEach((filter, i) => {
                filter.gain.value = values[i]
            })
        }

        localStorage.setItem("player_eq", JSON.stringify(values))

        // When setting numbers manually from UI, it's 'Custom'
        activePreset.value = "Custom"
        localStorage.setItem("player_eq_preset", "Custom")
    }

    function applyPreset(name) {
        if (presets[name]) {
            eqValues.value = [...presets[name]]

            if (eqInitialized && filters.length === 20) {
                filters.forEach((filter, i) => {
                    filter.gain.value = presets[name][i]
                })
            }

            localStorage.setItem("player_eq", JSON.stringify(presets[name]))
            activePreset.value = name
            localStorage.setItem("player_eq_preset", name)
        }
    }

    // Actions
    function setPlaylist(list) {
        playlist.value = list
    }

    function play(track = null) {
        if (!playlist.value.length) return

        if (!eqInitialized && Howler.ctx) {
            initEQ()
        }

        if (track) {
            const index = playlist.value.findIndex(t => t.path === track.path)
            if (index !== -1) {
                currentIndex.value = index
            }
        }

        const trackToPlay = playlist.value[currentIndex.value]

        // If resuming same track
        if (currentTrack.value && trackToPlay.path === currentTrack.value.path && sound) {
            if (!sound.playing()) {
                sound.volume(0)
                sound.play()
                sound.fade(0, 1.0, 150)
                isPlaying.value = true
                startSeekUpdate()
            }
            return
        }

        // Stop old sound
        if (sound) {
            sound.stop()
            sound.unload()
        }

        currentTrack.value = trackToPlay

        // Create new Howl
        // Note: html5 must be false for Web Audio API EQ to work
        sound = new Howl({
            src: [convertFileSrc(trackToPlay.path)],
            format: ['mp3', 'wav', 'ogg', 'flac', 'm4a'],
            html5: false,
            volume: 1.0,
            onend: onEnded,
            onload: () => {
                duration.value = sound.duration()
                // Ensure EQ is initialized if it wasn't before
                if (!eqInitialized) initEQ()
            },
            onplay: () => {
                isPlaying.value = true
                startSeekUpdate()
            },
            onpause: () => {
                isPlaying.value = false
                stopSeekUpdate()
            },
            onstop: () => {
                isPlaying.value = false
                stopSeekUpdate()
                currentTime.value = 0
            },
            onloaderror: (id, err) => {
                console.error("Load error", err)
                isPlaying.value = false
            },
            onplayerror: (id, err) => {
                console.error("Play error", err)
                isPlaying.value = false
            }
        })

        sound.play()
    }

    function pause() {
        if (sound && sound.playing()) {
            isPlaying.value = false
            stopSeekUpdate()

            const currentVol = sound.volume()
            sound.fade(currentVol, 0, 150)

            setTimeout(() => {
                if (!isPlaying.value) {
                    sound.pause()
                }
            }, 150)
        }
    }

    function toggle() {
        if (sound && sound.playing()) {
            pause()
        } else {
            play()
        }
    }

    function next() {
        if (!playlist.value.length) return

        if (shuffle.value) {
            currentIndex.value = Math.floor(Math.random() * playlist.value.length)
        } else {
            currentIndex.value = (currentIndex.value + 1) % playlist.value.length
        }
        play()
    }

    function prev() {
        if (!playlist.value.length) return
        currentIndex.value =
            (currentIndex.value - 1 + playlist.value.length) %
            playlist.value.length
        play()
    }

    function onEnded() {
        if (repeat.value) {
            if (sound) {
                sound.stop()
                sound.play()
            }
        } else {
            next()
        }
    }

    function setVolume(val) {
        volume.value = val
        Howler.volume(val * MAX_VOLUME_LIMIT)
        localStorage.setItem("player_volume", val)
    }

    function seek(val) {
        if (sound) {
            sound.seek(val)
            currentTime.value = val
        }
    }

    function startSeekUpdate() {
        stopSeekUpdate()
        seekInterval = setInterval(() => {
            if (sound && sound.playing()) {
                currentTime.value = sound.seek()
            }
        }, 500) // Update every 500ms
    }

    function stopSeekUpdate() {
        if (seekInterval) {
            clearInterval(seekInterval)
            seekInterval = null
        }
    }

    // Keep global volume in sync and save
    watch(volume, (val) => {
        Howler.volume(val * MAX_VOLUME_LIMIT)
        localStorage.setItem("player_volume", val)
    })

    watch(playlist, (newList) => {
        localStorage.setItem("player_playlist", JSON.stringify(newList))
    }, { deep: true })

    watch(currentIndex, (newIndex) => {
        localStorage.setItem("player_currentIndex", newIndex)
    })

    return {
        currentTrack,
        isPlaying,
        volume,
        playlist,
        currentIndex,
        repeat,
        shuffle,
        eqValues,
        activePreset,
        frequencies,
        presets,
        duration,
        currentTime,
        isDetailsVisible,
        play,
        pause,
        next,
        prev,
        toggle,
        setPlaylist,
        setVolume,
        setEQ,
        applyPreset,
        seek
    }
})
