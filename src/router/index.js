import { createRouter, createWebHistory } from "vue-router"

import Library from "../pages/Library.vue"
import Album from "../pages/Album.vue"
import AlbumSong from "../pages/AlbumSong.vue"
import Settings from "../pages/Settings.vue"
import Artists from "../pages/Artists.vue"
import ArtistDetails from "../pages/ArtistDetails.vue"
import Favorites from "../pages/favorites.vue"
import Test from "../pages/Test.vue"
import Playlist from "../pages/Playlist.vue"

const routes = [
    { path: "/", redirect: "/library" },
    { path: "/library", component: Library },
    { path: "/album", component: Album },
    { path: "/settings", component: Settings },
    { path: "/favorites", component: Favorites },
    { path: "/album/:name", component: AlbumSong },

    { path: "/artists", component: Artists },
    { path: "/artists/:name", name: "ArtistDetails", component: ArtistDetails },
    { path: "/playlist/:id", component: Playlist },
    { path: "/test", component: Test }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
