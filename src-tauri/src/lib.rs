mod db;
mod discord;
mod scan;
mod theaudiodb;

use std::sync::Mutex;

use discord::DiscordClient;
use sqlx::migrate::Migrator;
use tauri::Manager;
use tauri::State;

static MIGRATOR: Migrator = sqlx::migrate!("./src/migrations");

struct DiscordState {
    client: Mutex<DiscordClient>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            use tauri::async_runtime::block_on;
            // let start = std::time::Instant::now();
            let app_handle = app.handle().clone();

            block_on(async move {
                let data_dir = app_handle.path().app_data_dir().unwrap();
                std::fs::create_dir_all(&data_dir).unwrap();

                let db_path = data_dir.join("music.db");

                let pool = sqlx::sqlite::SqlitePoolOptions::new()
                    .connect_with(
                        sqlx::sqlite::SqliteConnectOptions::new()
                            .filename(&db_path)
                            .create_if_missing(true),
                    )
                    .await
                    .expect("db connect failed");

                MIGRATOR.run(&pool).await.expect("migrations failed");

                app_handle.manage(pool);
            });

            let discord = DiscordClient::new("1471960086325755924"); // Reemplaza con tu Client ID de Discord
            app.manage(DiscordState {
                client: Mutex::new(discord),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            scan::scan_folder,
            db::save_tracks,
            db::get_tracks,
            db::get_album_songs,
            db::add_music_folder,
            db::get_music_folders,
            db::get_all_artists,
            db::get_artist_songs,
            scan::get_lyrics,
            db::update_artist_image,
            update_presence,
            db::create_profile,
            db::delete_profile,
            db::update_profile,
            db::get_profiles,
            db::add_favorite,
            db::remove_favorite,
            db::get_favorites,
            db::create_playlist,
            db::get_playlists,
            db::delete_playlist,
            db::add_song_to_playlist,
            db::remove_song_from_playlist,
            db::get_playlist_songs,
            theaudiodb::fetch_artist_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}

#[tauri::command]
fn update_presence(
    title: String,
    artist: String,
    duration: i64,
    current_time: i64,
    playing: bool,
    state: State<DiscordState>,
) {
    let mut client = state.client.lock().unwrap();
    client.set_activity(&title, &artist, duration, current_time, playing);
}
