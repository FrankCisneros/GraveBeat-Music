import { defineStore } from "pinia"
import { invoke } from "@tauri-apps/api/core"
import { ref } from "vue"

export const useSettingsStore = defineStore("settings", () => {
    const folders = ref([])
    const theme = ref(localStorage.getItem("tema") || "cupcake")
    const availableThemes = ref([
        "light", "dark", "night", "emerald", "dracula", "wireframe", "black", "cupcake", "winter", "VelvetMist", "NeonOverdrive", "TokyoNight", "TokyoNightGhost", "gaming", "neon-dark"
    ])

    const activeProfileId = ref(
        parseInt(localStorage.getItem("profileId")) || 1
    )

    function setTheme(newTheme) {
        theme.value = newTheme
        localStorage.setItem("tema", newTheme)
        document.documentElement.setAttribute("data-theme", newTheme)
    }

    async function loadFolders() {
        folders.value = await invoke("get_music_folders", {
            profileId: activeProfileId.value
        })
    }

    async function addFolder(path) {
        await invoke("add_music_folder", {
            path,
            profileId: activeProfileId.value
        })
        await loadFolders()
    }

    async function removeFolder(index) {
        folders.value.splice(index, 1)
    }

    function setActiveProfile(id) {
        activeProfileId.value = id
        localStorage.setItem("profileId", id)
        loadFolders()
    }

    return {
        activeProfileId,
        folders,
        theme,
        setActiveProfile,
        loadFolders,
        addFolder,
        removeFolder,
        setTheme,
        availableThemes
    }
})
