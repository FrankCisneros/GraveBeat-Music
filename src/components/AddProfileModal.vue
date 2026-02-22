<template>
    <dialog id="add_profile_modal" class="modal text-base-content">
        <div class="modal-box w-11/12 max-w-2xl bg-base-100/95 backdrop-blur-md border border-base-300 shadow-2xl">
            <h3 class="font-bold text-2xl mb-6 text-center">Añadir Nuevo Perfil</h3>

            <div class="flex flex-col gap-6">
                <!-- Name Input -->
                <div class="form-control w-full">
                    <label class="label">
                        <span class="label-text font-medium ml-1">Nombre del Perfil</span>
                    </label>
                    <input type="text" v-model="profileName" placeholder="Ej. Rock, Relax, Gaming"
                        class="input input-bordered w-full bg-base-200 focus:bg-base-100 transition-all font-medium text-lg" />
                </div>

                <!-- Image Upload & Crop -->
                <div class="flex flex-col items-center gap-4">
                    <label class="label w-full pb-0">
                        <span class="label-text font-medium ml-1">Avatar Personalizado</span>
                    </label>

                    <div v-if="!imageSrc"
                        class="w-full h-64 bg-base-200/50 rounded-xl border-2 border-dashed border-base-content/20 hover:border-primary/50 hover:bg-base-200 transition-all cursor-pointer flex flex-col items-center justify-center gap-3 group"
                        @click="triggerFileInput">
                        <div class="p-4 rounded-full bg-base-300 group-hover:scale-110 transition-transform">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                stroke="currentColor" class="w-8 h-8 opacity-70">
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M6.827 6.175A2.31 2.31 0 015.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 00-1.134-.175 2.31 2.31 0 01-1.64-1.055l-.822-1.316a2.192 2.192 0 00-1.736-1.039 48.774 48.774 0 00-5.232 0 2.192 2.192 0 00-1.736 1.039l-.821 1.316z" />
                                <path stroke-linecap="round" stroke-linejoin="round"
                                    d="M16.5 12.75a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0zM18.75 10.5h.008v.008h-.008V10.5z" />
                            </svg>
                        </div>
                        <p class="text-sm font-medium opacity-60 group-hover:opacity-100 transition-opacity">Click para
                            subir una imagen</p>
                    </div>

                    <div v-else class="w-full flex flex-col items-center gap-4">
                        <div
                            class="w-full h-80 rounded-xl overflow-hidden shadow-inner bg-neutral-900 border border-base-300">
                            <Cropper ref="cropperRef" class="cropper h-full" :src="imageSrc" :stencil-props="{
                                aspectRatio: 1,
                                handlers: {},
                                movable: true,
                                scalable: true,
                                resizable: true
                            }" :stencil-component="CircleStencil" image-restriction="stencil" />
                        </div>
                        <div class="flex gap-2">
                            <button class="btn btn-sm btn-ghost text-error" @click="imageSrc = null">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                    stroke-width="1.5" stroke="currentColor" class="w-4 h-4 mr-1">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                        d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                </svg>
                                Eliminar
                            </button>
                            <button class="btn btn-sm btn-secondary btn-outline" @click="triggerFileInput">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                    stroke-width="1.5" stroke="currentColor" class="w-4 h-4 mr-1">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                </svg>
                                Cambiar
                            </button>
                        </div>
                    </div>
                    <input type="file" ref="fileInput" class="hidden" accept="image/*" @change="onFileChange" />
                </div>
            </div>

            <div class="modal-action border-t border-base-200 mt-6 pt-4">
                <form method="dialog" class="flex gap-2 w-full justify-end">
                    <button class="btn btn-ghost px-6">Cancelar</button>
                    <button class="btn btn-primary px-8" @click.prevent="saveProfile"
                        :disabled="!profileName || loading">
                        <span v-if="loading" class="loading loading-spinner loading-sm mr-2"></span>
                        {{ loading ? 'Creando...' : 'Crear Perfil' }}
                    </button>
                </form>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button>close</button>
        </form>
    </dialog>
</template>

<script setup>
import { ref } from 'vue';
import { Cropper, CircleStencil } from 'vue-advanced-cropper';
import 'vue-advanced-cropper/dist/style.css';
import { invoke } from '@tauri-apps/api/core';
import { writeFile, BaseDirectory, mkdir } from '@tauri-apps/plugin-fs';
import { appDataDir, join } from '@tauri-apps/api/path';

const emit = defineEmits(['profile-added']);

const profileName = ref('');
const imageSrc = ref(null);
const fileInput = ref(null);
const cropperRef = ref(null);
const loading = ref(false);

const triggerFileInput = () => {
    fileInput.value.click();
};

const onFileChange = (e) => {
    const file = e.target.files[0];
    if (file) {
        if (imageSrc.value) {
            URL.revokeObjectURL(imageSrc.value);
        }
        imageSrc.value = URL.createObjectURL(file);
    }
};

const saveProfile = async () => {
    if (!profileName.value) return;
    loading.value = true;

    try {
        let avatarPath = null;

        if (cropperRef.value && imageSrc.value) {
            const { canvas } = cropperRef.value.getResult();
            if (canvas) {
                const blob = await new Promise(resolve => canvas.toBlob(resolve, 'image/png'));
                const arrayBuffer = await blob.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);

                try {
                    await mkdir('avatars', { baseDir: BaseDirectory.AppData, recursive: true });
                } catch (e) {
                    // Ignore if exists
                }

                const filename = `avatars/${Date.now()}_${profileName.value.replace(/\s+/g, '_').toLowerCase()}.png`;
                await writeFile(filename, uint8Array, { baseDir: BaseDirectory.AppData });

                const appDataPath = await appDataDir();
                avatarPath = await join(appDataPath, filename);
            }
        }

        await invoke("create_profile", {
            name: profileName.value,
            avatarPath: avatarPath
        });

        profileName.value = '';
        imageSrc.value = null;
        document.getElementById('add_profile_modal').close();
        emit('profile-added');

    } catch (e) {
        console.error("Error creating profile:", e);
    } finally {
        loading.value = false;
    }
};

defineExpose({
    show: () => document.getElementById('add_profile_modal').showModal()
});
</script>
