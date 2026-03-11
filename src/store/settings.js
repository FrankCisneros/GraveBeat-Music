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

    const isScanning = ref(false)
    const scanProgress = ref(0)
    const scanTotal = ref(0)
    const scanMessage = ref("")

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

    async function performScan() {
        if (isScanning.value) return;
        isScanning.value = true;
        scanProgress.value = 0;
        scanTotal.value = 0;
        scanMessage.value = "Iniciando escaneo...";

        const { listen } = await import('@tauri-apps/api/event');
        const unlisten = await listen('scan-progress', (event) => {
            scanProgress.value = event.payload.current;
            scanTotal.value = event.payload.total;
        });

        try {
            await loadFolders();
            let foundAny = false;
            for (const folder of folders.value) {
                scanMessage.value = `Escaneando ${folder}`;
                const result = await invoke("scan_folder", { path: folder });
                if (result?.length) {
                    foundAny = true;
                    await invoke("save_tracks", {
                        tracks: result,
                        profileId: activeProfileId.value
                    });
                }
            }
            localStorage.setItem(`lastScan_${activeProfileId.value}`, Math.floor(Date.now() / 1000).toString());
            return foundAny;
        } catch (e) {
            console.error(e);
            throw e;
        } finally {
            isScanning.value = false;
            unlisten();
        }
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
        availableThemes,
        isScanning,
        scanProgress,
        scanTotal,
        scanMessage,
        performScan
    }
})
