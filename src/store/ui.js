import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUIStore = defineStore('ui', () => {
    const contextMenu = ref({
        visible: false,
        x: 0,
        y: 0,
        track: null,
        items: []
    })

    function showContextMenu(e, track, customItems = null) {
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
            items: customItems
        }
    }

    function hideContextMenu() {
        contextMenu.value.visible = false
    }

    return {
        contextMenu,
        showContextMenu,
        hideContextMenu
    }
})
