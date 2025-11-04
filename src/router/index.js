import { createRouter, createWebHistory } from "vue-router"

import Library from "../pages/Library.vue"
import files from "../pages/files.vue"
// import NowPlaying from "../pages/NowPlaying.vue"
// import Settings from "../pages/Settings.vue"
// import About from "../pages/About.vue"

const routes = [
    { path: "/", redirect: "/library" },
    { path: "/library", component: Library },
    { path: "/files", component: files },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
