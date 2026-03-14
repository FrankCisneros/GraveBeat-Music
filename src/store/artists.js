import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from './settings';

export const useArtistsStore = defineStore('artists', {
    state: () => ({
        artists: [], // { name, image_path, bio, song_count }
        currentArtistSongs: [],
        loading: false
    }),

    actions: {
        async fetchArtists() {
            this.loading = true;
            const settingsStore = useSettingsStore();
            try {
                this.artists = await invoke('get_all_artists', {
                    profileId: settingsStore.activeProfileId
                });
            } catch (error) {
                console.error("Error fetching artists:", error);
            } finally {
                this.loading = false;
            }
        },

        async fetchArtistSongs(artistName) {
            this.loading = true;
            const settingsStore = useSettingsStore();
            try {
                this.currentArtistSongs = await invoke('get_artist_songs', {
                    profileId: settingsStore.activeProfileId,
                    artist: artistName
                });
                return this.currentArtistSongs;
            } catch (error) {
                console.error("Error fetching artist songs:", error);
                return [];
            } finally {
                this.loading = false;
            }
        },

        async updateArtistImage(artistName, imageUrl) {
            const settingsStore = useSettingsStore();
            try {
                await invoke('update_artist_image', {
                    profileId: settingsStore.activeProfileId,
                    artist: artistName,
                    imagePath: imageUrl
                });

                // Update local state immediately
                const artist = this.artists.find(a => a.name === artistName);
                if (artist) {
                    artist.image_path = imageUrl;
                }
            } catch (error) {
                console.error("Error updating artist image:", error);
            }
        },

        async ensureArtistImage(artist) {
            if (artist.image_path) return;

            // 1. Try TheAudioDB via Rust Backend
            try {
                const imageUrl = await invoke('fetch_artist_image', { artistName: artist.name });

                if (imageUrl) {
                    await this.updateArtistImage(artist.name, imageUrl);
                    return;
                }
            } catch (error) {
                console.error("Error fetching artist image from Spotify:", error);
            }

            // 2. Fallback to song cover
            try {
                const songs = await this.fetchArtistSongs(artist.name);
                if (songs.length > 0 && songs[0].cover_path) {
                    // Use song cover as fallback
                    await this.updateArtistImage(artist.name, songs[0].cover_path);
                }
            } catch (e) {
                console.error("Error finding fallback cover", e);
            }
        }
    }
});
