<template>
    <div class="space-y-8 pb-8">
        <!-- Header -->
        <div class="flex justify-between items-end">
            <div>
                <h1 class="text-4xl font-bold text-base-content mb-2">Configuración</h1>
                <p class="text-base-content/70">Personaliza tu experiencia de música</p>
            </div>
            <div class="text-sm breadcrumbs hidden md:inline-flex">
                <ul>
                    <li><a>Inicio</a></li>
                    <li>Configuración</li>
                </ul>
            </div>
        </div>

        <!-- Main Grid -->
        <div class="grid grid-cols-1 xl:grid-cols-2 gap-8">

            <!-- Library Management Section -->
            <div class="space-y-8">
                <!-- Music Folders -->
                <div class="card bg-base-100 shadow-xl border border-base-200">
                    <div class="card-body">
                        <h2 class="card-title text-xl mb-4 flex items-center gap-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-info" fill="none"
                                viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1l-5 5h-5l5-5" />
                            </svg>
                            Biblioteca Musical
                        </h2>

                        <!-- Scan Action -->
                        <div class="flex items-center justify-between p-4 bg-base-200/50 rounded-lg mb-6">
                            <div>
                                <h3 class="font-semibold">Escanear Biblioteca</h3>
                                <p class="text-xs text-base-content/60">Actualiza tu lista de canciones</p>
                            </div>
                            <div class="flex">
                                <button @click="scanAll" class="btn btn-outline btn-info" :disabled="loading">
                                    <span v-if="loading" class="loading loading-spinner loading-sm text-white"></span>
                                    {{ loading ? 'Escaneando...' : 'Escanear Ahora' }}
                                </button>
                                <button class="btn btn-outline btn-error ml-2" @click="clearLibrary">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24"
                                        stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                        
                        <div v-if="scanError" class="alert alert-error text-sm mb-4">
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                                viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            {{ scanError }}
                        </div>
                        <div v-if="scanSuccess" class="alert alert-success text-sm mb-4">
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none"
                                viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            Biblioteca actualizada correctamente.
                        </div>

                        <div class="divider">Gestión de Carpetas</div>

                        <!-- Add Folder -->
                        <div class="form-control mb-4">
                            <div class="flex gap-2">
                                <input ref="folderInput" type="file" webkitdirectory directory class="hidden"
                                    @change="handleFolderSelect" />
                                <button @click="openFolderPicker" class="btn btn-outline btn-info gap-2 max-w-xs">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20"
                                        fill="currentColor">
                                        <path d="M2 6a2 2 0 012-2h12a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
                                    </svg>
                                    Seleccionar Carpeta
                                </button>
                                <div class="join flex-1 min-w-0">
                                    <input v-model="newFolderPath" type="text" placeholder="Ruta manual..."
                                        class="input input-bordered join-item w-full" @keyup.enter="addNewFolder" />
                                    <button @click="addNewFolder" class="btn btn-square join-item"
                                        :disabled="!newFolderPath.trim()">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20"
                                            fill="currentColor">
                                            <path fill-rule="evenodd"
                                                d="M10 5a1 1 0 011 1v3h3a1 1 0 110 2h-3v3a1 1 0 11-2 0v-3H6a1 1 0 110-2h3V6a1 1 0 011-1z"
                                                clip-rule="evenodd" />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </div>

                        <!-- Folder List -->
                        <div class="bg-base-200 rounded-box p-2 max-h-48 overflow-y-auto">
                            <div v-if="settingsStore.folders.length === 0"
                                class="flex flex-col items-center justify-center py-8 text-base-content/40 text-sm">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 mb-2 opacity-50" fill="none"
                                    viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                        d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
                                </svg>
                                <span>Sin carpetas configuradas</span>
                            </div>
                            <div v-else v-for="(folder, index) in settingsStore.folders" :key="index"
                                class="flex items-center justify-between p-3 hover:bg-base-500 rounded-lg transition-colors group">
                                <div class="flex items-center gap-3 overflow-hidden">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-info shrink-0"
                                        viewBox="0 0 20 20" fill="currentColor">
                                        <path d="M2 6a2 2 0 012-2h12a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" />
                                    </svg>
                                    <span class="text-xs truncate font-mono opacity-80" :title="folder">{{ folder
                                    }}</span>
                                </div>
                                <button @click="removeFolder(folder)"
                                    class="btn btn-ghost btn-xs btn-square text-error opacity-0 group-hover:opacity-100 transition-opacity">
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20"
                                        fill="currentColor">
                                        <path fill-rule="evenodd"
                                            d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                                            clip-rule="evenodd" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Appearance & Preferences -->
            <div class="space-y-8">
                <!-- Appearance -->
                <div class="card bg-base-100 shadow-xl border border-base-200">
                    <div class="card-body">
                        <h2 class="card-title text-xl mb-4 flex items-center gap-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-info" fill="none"
                                viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.5a2 2 0 00-1 .267V5a2 2 0 10-4 0v.75" />
                            </svg>
                            Apariencia
                        </h2>

                        <div class="form-control w-full">
                            <label class="label">
                                <span class="label-text">Tema de la interfaz</span>
                                <span class="label-text-alt opacity-50">Vista previa instantánea</span>
                            </label>
                            <div class="flex gap-4 items-center">
                                <select class="select select-bordered w-full capitalize" :value="settingsStore.theme"
                                    @change="e => settingsStore.setTheme(e.target.value)">
                                    <option v-for="theme in settingsStore.availableThemes" :key="theme" :value="theme">
                                        {{ theme }}
                                    </option>
                                </select>

                                <div class="bg-base-100 grid shrink-0 grid-cols-2 gap-1 rounded-md p-1 shadow-sm">
                                    <div class="bg-base-content size-2 rounded-full"></div>
                                    <div class="bg-primary size-2 rounded-full"></div>
                                    <div class="bg-secondary size-2 rounded-full"></div>
                                    <div class="bg-accent size-2 rounded-full"></div>
                                </div>

                                <!--<div class="w-10 h-10 rounded-lg shadow-sm border border-base-content/10 grid"
                                    :data-theme="settingsStore.theme" :class="'bg-base-100'"></div> -->
                            </div>
                        </div>
                    </div>
                </div>

                <!-- App Preferences -->
                <div class="card bg-base-100 shadow-xl border border-base-200">
                    <div class="card-body">
                        <h2 class="card-title text-xl mb-4 flex items-center gap-2">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-info" fill="none"
                                viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                    d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V8m12 6a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V8m-6 12h.01M12 18a2 2 0 100-4m0 4a2 2 0 110-4" />
                            </svg>
                            Preferencias de Reproducción
                        </h2>

                        <div class="form-control">
                            <label class="label cursor-pointer justify-start gap-4">
                                <input type="checkbox" class="toggle toggle-primary" checked />
                                <span class="label-text">Crossfade (Transición suave)</span>
                            </label>
                        </div>

                        <div class="form-control">
                            <label class="label cursor-pointer justify-start gap-4">
                                <input type="checkbox" class="toggle toggle-primary" checked />
                                <span class="label-text">Normalizar volumen</span>
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue"
import { useSettingsStore } from "../store/settings"
import { invoke } from "@tauri-apps/api/core"

const settingsStore = useSettingsStore()
const newFolderPath = ref("")
const folderInput = ref(null)

const loading = ref(false)
const scanError = ref("")
const scanSuccess = ref(false)

onMounted(() => {
    settingsStore.loadFolders()
})

const openFolderPicker = () => {
    // Attempt to trigger the file input
    if (folderInput.value) {
        folderInput.value.click();
    }
}

const handleFolderSelect = (event) => {
    const files = event.target.files
    if (files && files.length > 0) {
        const firstFile = files[0]
        let folderPath = firstFile.webkitRelativePath || firstFile.path || ""

        if (folderPath) {
            folderPath = folderPath.substring(0, folderPath.lastIndexOf("/"))
            // Cleanup path string if needed
            if (folderPath) {
                settingsStore.addFolder(folderPath)
            }
        }

        // Reset input to allow selecting same folder again if needed
        event.target.value = ''
    }
}

const addNewFolder = () => {
    if (newFolderPath.value.trim()) {
        settingsStore.addFolder(newFolderPath.value.trim())
        newFolderPath.value = ""
    }
}

const removeFolder = (folderPath) => {
    settingsStore.removeFolder(folderPath)
}

const scanAll = async () => {
    loading.value = true
    scanError.value = ""
    scanSuccess.value = false

    try {
        await settingsStore.loadFolders()

        let foundAny = false
        // Escaneamos y guardamos por carpetas
        for (const folder of settingsStore.folders) {
            const result = await invoke("scan_folder", { path: folder })

            if (result?.length) {
                foundAny = true
                await invoke("save_tracks", {
                    tracks: result,
                    profileId: settingsStore.activeProfileId
                })
            }
        }

        // Slight delay to show loading state
        setTimeout(() => {
            loading.value = false
            if (foundAny) {
                scanSuccess.value = true
                setTimeout(() => scanSuccess.value = false, 3000)
            } else {
                scanError.value = "No se encontraron canciones."
            }
        }, 500)

    } catch (err) {
        console.error(err)
        scanError.value = "Error durante el escaneo: " + err
        loading.value = false
    }
}
</script>

<style scoped>
/* Scoped styles if needed */
</style>
