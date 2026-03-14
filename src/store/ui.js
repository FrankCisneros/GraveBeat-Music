import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUIStore = defineStore('ui', () => {
    const contextMenu = ref({
        visible: false,
        x: 0,
        y: 0,
        track: null,
        items: [],
        playlistId: null
    })

    const playlistModal = ref({
        visible: false,
        track: null
    })

    function showContextMenu(e, track, customItems = null, playlistId = null) {
        e.preventDefault()
        e.stopPropagation()

        let menuWidth = 220; // Approx
        let menuHeight = 200; // Approx

        // Calculate position to keep within viewport
        let x = e.clientX
        let y = e.clientY

        if (x + menuWidth > window.innerWidth) {
            x = x - menuWidth
        }

        if (y + menuHeight > window.innerHeight) {
            y = y - menuHeight
        }

        contextMenu.value = {
            visible: true,
            x,
            y,
            track,
            items: customItems,
            playlistId
        }
    }

    function hideContextMenu() {
        contextMenu.value.visible = false
    }

    function showPlaylistModalAction(track) {
        playlistModal.value.visible = true
        playlistModal.value.track = track
    }

    function hidePlaylistModal() {
        playlistModal.value.visible = false
        playlistModal.value.track = null
    }

    return {
        contextMenu,
        playlistModal,
        showContextMenu,
        hideContextMenu,
        showPlaylistModalAction,
        hidePlaylistModal
    }
})
