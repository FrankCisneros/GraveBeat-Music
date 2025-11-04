<template>
    <div>
        <h1 class="text-2xl font-bold mb-4">🎶 Biblioteca local</h1>

        <div class="flex gap-2 mb-4">
            <input v-model="dir" class="input input-bordered w-96"
                placeholder="Ruta de carpeta (ej: C:\\Users\\gungr\\Music)" />
            <button class="btn btn-primary" @click="startScan" :disabled="loading">
                {{ loading ? 'Escaneando...' : 'Escanear' }}
            </button>
        </div>

        <div v-if="error" class="text-red-500 mb-4">{{ error }}</div>

        <div v-if="tracks.length" class="grid grid-cols-4 gap-4 mt-4">
            <div v-for="t in tracks" :key="t.path" class="card p-2 bg-base-200">
                <img :src="t.cover_base64 || 'https://placehold.co/200x200?text=No+Cover'"
                    class="w-full h-36 object-cover mb-2 rounded-lg" />
                <div class="font-bold truncate">{{ t.title }}</div>
                <div class="text-xs text-gray-500 truncate">{{ t.artist }}</div>
                <div class="text-xs text-gray-400 truncate">{{ t.album }}</div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"

const dir = ref("C:\\Users\\gungr\\Music") // Ruta inicial
const tracks = ref([])
const loading = ref(false)
const error = ref("")

async function startScan() {
    error.value = ""
    loading.value = true
    tracks.value = []

    try {
        const result = await invoke("scan_music_folder", { folderPath: dir.value })
        tracks.value = result
    } catch (err) {
        error.value = `Error: ${err}`
    } finally {
        loading.value = false
    }
}
</script>
