<template>
    <nav data-tauri-drag-region class="navbar bg-base-200 min-h-14 h-14 p-0 shadow-md select-none drag-region">

        <!-- Left: Spacer for balance (or Logo in future) -->
        <div class="navbar-start flex-1 flex items-center gap-2 px-4">
            <!-- Optional: App Title could go here -->
            <img src="@/assets/icon.png" class="w-8 h-8 rounded-full"/>
            <span class="ml-4 text-xs font-bold opacity-40 uppercase tracking-widest hidden lg:block">
                Distro Music Player
            </span>
        </div>

        <!-- Center: Search Bar (Wider) -->
        <div class="navbar-center flex-2 w-full max-w-3xl px-4">
            <div class="join w-full shadow-sm">
                <input type="text" placeholder="Buscar música..."
                    class="input input-sm w-full join-item focus:outline-none bg-base-100/50 focus:bg-base-100 transition-all text-center focus:text-left" />
                <button class="btn btn-sm join-item btn-ghost bg-base-100/50 hover:bg-base-100">
                    <svg class="h-4 w-4 opacity-70" viewBox="0 0 24 24">
                        <path d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                </button>
            </div>
        </div>

        <!-- Right: Profile & Window Controls -->
        <div class="navbar-end flex-1 flex items-center justify-end gap-3 pr-2">

            <!-- User Profile -->
            <div class="dropdown dropdown-end">
                <button tabindex="0" class="btn btn-ghost btn-circle btn-sm avatar">
                    <div
                        class="w-8 rounded-full bg-neutral text-neutral-content ring ring-base-300 ring-offset-base-100 ring-offset-1">
                        <img v-if="activeProfile?.avatar_path" :src="getProfileImage(activeProfile.avatar_path)" />
                        <span v-else class="text-xs text-center justify-center items-center flex h-full">{{
                            activeProfile?.name?.charAt(0).toUpperCase() || 'U' }}</span>
                    </div>
                </button>

                <ul tabindex="0"
                    class="menu dropdown-content bg-base-100 rounded-box shadow-xl border border-base-300 w-60 mt-4 p-2 z-50">
                    <li class="menu-title opacity-70">
                        <span>Perfiles</span>
                    </li>

                    <div class="max-h-[220px] overflow-y-auto custom-scrollbar">
                        <li v-for="profile in profiles" :key="profile.id">
                            <a class="flex items-center gap-2" :class="{ 'active': profile.id === activeProfile?.id }"
                                @click="selectProfile(profile)">
                                <div class="avatar placeholder">
                                    <div class="w-6 rounded-full bg-neutral text-neutral-content">
                                        <img v-if="profile.avatar_path" :src="getProfileImage(profile.avatar_path)" />
                                        <span v-else
                                            class="text-xs text-center justify-center items-center flex h-full">{{
                                                profile.name.charAt(0).toUpperCase() }}</span>
                                    </div>
                                </div>
                                <span class="truncate flex-1">{{ profile.name }}</span>
                                <span v-if="profile.name === 'Default'"
                                    class="badge badge-xs badge-ghost">Default</span>
                            </a>
                        </li>
                    </div>

                    <div class="divider my-1"></div>
                    <li>
                        <a class="flex items-center gap-2 text-primary font-medium" @click="openAddProfileModal">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
                                class="w-5 h-5">
                                <path
                                    d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
                            </svg>
                            Añadir Perfil
                        </a>
                    </li>
                </ul>
            </div>

            <!-- Divider -->
            <div class="h-6 w-px bg-base-content/10 mx-1"></div>

            <!-- Window Controls -->
            <div class="flex items-center gap-1">
                <button @click="minimize"
                    class="btn btn-square btn-ghost btn-sm bg-transparent hover:bg-base-300 h-8 w-8 min-h-0 border-none"
                    title="Minimize">
                    <svg class="h-4 w-4" viewBox="0 0 24 24">
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                </button>
                <button @click="maximize"
                    class="btn btn-square btn-ghost btn-sm bg-transparent hover:bg-base-300 h-8 w-8 min-h-0 border-none"
                    title="Maximize">
                    <svg class="h-4 w-4" viewBox="0 0 24 24">
                        <rect x="5" y="5" width="14" height="14" rx="2" />
                    </svg>
                </button>
                <button @click="close"
                    class="btn btn-square btn-ghost btn-sm bg-transparent hover:bg-error hover:text-white h-8 w-8 min-h-0 border-none"
                    title="Close">
                    <svg class="h-4 w-4" viewBox="0 0 24 24">
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                </button>
            </div>
        </div>
    </nav>

    <AddProfileModal ref="addProfileModalRef" @profile-added="fetchProfiles" />
</template>


<script setup>
import { computed, ref, onMounted, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import AddProfileModal from './AddProfileModal.vue';

import { useSettingsStore } from "../store/settings";

const appWindow = getCurrentWindow();
const settingsStore = useSettingsStore();

// Props para recibir el tema del padre
const props = defineProps(["modelValue", "temas"]);
const emit = defineEmits(["update:modelValue"]);

const profiles = ref([]);
const activeProfile = ref(null);
const addProfileModalRef = ref(null);

// Sincronizar el select con el padre
const internalTema = computed({
    get: () => props.modelValue,
    set: (val) => emit("update:modelValue", val),
});

// Funciones de control de ventana
const minimize = () => appWindow.minimize();
const maximize = () => appWindow.toggleMaximize();
const close = () => appWindow.close();

watch(() => internalTema.value, () => {
    emit("update:modelValue", internalTema.value);
});

const getProfileImage = (path) => {
    return path ? convertFileSrc(path) : null;
};

const openAddProfileModal = () => {
    addProfileModalRef.value?.show();
};

const fetchProfiles = async () => {
    try {
        profiles.value = await invoke("get_profiles");

        // Find the active profile based on store ID
        if (profiles.value.length > 0) {
            const storedId = settingsStore.activeProfileId;
            activeProfile.value = profiles.value.find(p => p.id === storedId) || profiles.value[0];

            // Sync store if we fell back to default
            if (activeProfile.value.id !== storedId) {
                settingsStore.setActiveProfile(activeProfile.value.id);
            }
        }
    } catch (e) {
        console.error("Failed to fetch profiles", e);
    }
};

const selectProfile = (profile) => {
    activeProfile.value = profile;
    settingsStore.setActiveProfile(profile.id);
};

// Update active profile if store changes (e.g. from another component)
watch(() => settingsStore.activeProfileId, (newId) => {
    if (profiles.value.length > 0) {
        activeProfile.value = profiles.value.find(p => p.id === newId) || activeProfile.value;
    }
});

onMounted(async () => {
    await fetchProfiles();
});
</script>

<style scoped>
.drag-region {
    -webkit-app-region: drag;
}

/* Ensure buttons are clickable and not part of the drag region */
button,
.dropdown,
input,
a {
    -webkit-app-region: no-drag;
}

svg {
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
    stroke-linecap: round;
    stroke-linejoin: round;
}

.custom-scrollbar::-webkit-scrollbar {
    width: 4px;
}

.custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
    background: #e5e7eb;
    /* base-300 approx */
    border-radius: 4px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: #d1d5db;
}
</style>
