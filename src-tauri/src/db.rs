use crate::scan::TrackMetadata;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct ArtistMetadata {
    pub name: String,
    pub image_path: Option<String>,
    pub bio: Option<String>,
    pub song_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub avatar_path: Option<String>,
    pub settings: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_path: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub song_count: i32,
}

#[tauri::command]
pub async fn save_tracks(
    pool: State<'_, SqlitePool>,
    tracks: Vec<TrackMetadata>,
    profile_id: i32,
) -> Result<(), String> {
    if tracks.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Dividimos en trozos (chunks) para no exceder el límite de variables de SQLite
    for chunk in tracks.chunks(100) {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT OR REPLACE INTO songs (path, name, title, artist, album, duration, cover_path, profile_id) "
        );

        query_builder.push_values(chunk, |mut b, t| {
            b.push_bind(&t.path)
                .push_bind(&t.name)
                .push_bind(&t.title)
                .push_bind(&t.artist)
                .push_bind(&t.album)
                .push_bind(t.duration)
                .push_bind(&t.cover_path)
                .push_bind(profile_id);
        });

        let query = query_builder.build();
        query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }

    // Actualizar tabla de artistas con los nuevos artistas insertados
    sqlx::query(
        "INSERT OR IGNORE INTO artists (name, profile_id)
        SELECT DISTINCT artist, profile_id FROM songs WHERE profile_id = ? AND artist IS NOT NULL",
    )
    .bind(profile_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_tracks(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
        EXISTS(SELECT 1 FROM favorites f WHERE f.song_id = s.id AND f.profile_id = s.profile_id) as is_favorite
        FROM songs s
        WHERE s.profile_id = ?
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(profile_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(row.get("is_favorite")),
        })
        .collect();

    Ok(tracks)
}

#[tauri::command]
pub async fn get_album_songs(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    album: String,
) -> Result<Vec<TrackMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
        EXISTS(SELECT 1 FROM favorites f WHERE f.song_id = s.id AND f.profile_id = s.profile_id) as is_favorite
        FROM songs s
        WHERE s.profile_id = ? AND s.album = ?
        ORDER BY s.name ASC
        "#,
    )
    .bind(profile_id)
    .bind(album)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(row.get("is_favorite")),
        })
        .collect();

    Ok(tracks)
}

#[tauri::command]
pub async fn add_music_folder(
    pool: State<'_, SqlitePool>,
    path: String,
    profile_id: i32,
) -> Result<(), String> {
    sqlx::query(
        "INSERT OR IGNORE INTO music_folders (path, added_at, profile_id) VALUES (?1, strftime('%s','now'), ?2)",
    )
    .bind(path)
    .bind(profile_id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_music_folders(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
) -> Result<Vec<String>, String> {
    let rows =
        sqlx::query("SELECT path FROM music_folders WHERE profile_id = ? ORDER BY added_at ASC")
            .bind(profile_id)
            .fetch_all(&*pool)
            .await
            .map_err(|e| e.to_string())?;

    let paths = rows.into_iter().map(|row| row.get("path")).collect();

    Ok(paths)
}

#[tauri::command]
pub async fn get_all_artists(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
) -> Result<Vec<ArtistMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT a.name, a.image_path, a.bio, COUNT(s.id) as song_count
        FROM artists a
        LEFT JOIN songs s ON s.artist = a.name AND s.profile_id = a.profile_id
        WHERE a.profile_id = ?
        GROUP BY a.name
        ORDER BY a.name ASC
        "#,
    )
    .bind(profile_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let artists = rows
        .into_iter()
        .map(|row| ArtistMetadata {
            name: row.get("name"),
            image_path: row.get("image_path"),
            bio: row.get("bio"),
            song_count: row.get("song_count"),
        })
        .collect();

    Ok(artists)
}

#[tauri::command]
pub async fn get_artist_songs(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    artist: String,
) -> Result<Vec<TrackMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
        EXISTS(SELECT 1 FROM favorites f WHERE f.song_id = s.id AND f.profile_id = s.profile_id) as is_favorite
        FROM songs s
        WHERE s.profile_id = ? AND s.artist = ?
        ORDER BY s.title ASC
        "#,
    )
    .bind(profile_id)
    .bind(artist)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(row.get("is_favorite")),
        })
        .collect();

    Ok(tracks)
}

#[tauri::command]
pub async fn update_artist_image(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    artist: String,
    image_path: String,
) -> Result<(), String> {
    sqlx::query("UPDATE artists SET image_path = ? WHERE name = ? AND profile_id = ?")
        .bind(image_path)
        .bind(artist)
        .bind(profile_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_profile(
    pool: State<'_, SqlitePool>,
    name: String,
    avatar_path: Option<String>,
) -> Result<(), String> {
    sqlx::query("INSERT INTO profiles (name, avatar_path, created_at, updated_at) VALUES (?, ?, strftime('%s','now'), strftime('%s','now'))")
        .bind(name)
        .bind(avatar_path)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_profile(pool: State<'_, SqlitePool>, profile_id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM profiles WHERE id = ?")
        .bind(profile_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_profile(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    name: String,
    avatar_path: Option<String>,
) -> Result<(), String> {
    sqlx::query("UPDATE profiles SET name = ?, avatar_path = ?, updated_at = strftime('%s','now') WHERE id = ?")
        .bind(name)
        .bind(avatar_path)
        .bind(profile_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_profiles(pool: State<'_, SqlitePool>) -> Result<Vec<Profile>, String> {
    let rows = sqlx::query(
        "SELECT id, name, avatar_path, settings, created_at, updated_at FROM profiles ORDER BY CASE WHEN name = 'Default' THEN 0 ELSE 1 END, name ASC",
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    let profiles = rows
        .into_iter()
        .map(|row| Profile {
            id: row.get("id"),
            name: row.get("name"),
            avatar_path: row.get("avatar_path"),
            settings: row.get("settings"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();
    Ok(profiles)
}

#[tauri::command]
pub async fn add_favorite(
    pool: State<'_, SqlitePool>,
    song_id: i32,
    profile_id: i32,
) -> Result<(), String> {
    sqlx::query("INSERT OR IGNORE INTO favorites (song_id, profile_id) VALUES (?1, ?2)")
        .bind(song_id)
        .bind(profile_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn remove_favorite(
    pool: State<'_, SqlitePool>,
    song_id: i32,
    profile_id: i32,
) -> Result<(), String> {
    sqlx::query("DELETE FROM favorites WHERE song_id = ? AND profile_id = ?")
        .bind(song_id)
        .bind(profile_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_favorites(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
) -> Result<Vec<TrackMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
               1 as is_favorite
        FROM songs s
        INNER JOIN favorites f ON f.song_id = s.id
        WHERE f.profile_id = ?
        ORDER BY f.id ASC
        "#,
    )
    .bind(profile_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(true),
        })
        .collect();
    Ok(tracks)
}

#[tauri::command]
pub async fn create_playlist(
    pool: State<'_, SqlitePool>,
    name: String,
    description: Option<String>,
    profile_id: i32,
) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO playlists (name, description, created_at, updated_at, profile_id) VALUES (?, ?, strftime('%s','now'), strftime('%s','now'), ?)",
    )
    .bind(name)
    .bind(description)
    .bind(profile_id)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_playlists(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
) -> Result<Vec<Playlist>, String> {
    let rows = sqlx::query(
        r#"
        SELECT p.id, p.name, p.description, p.cover_path, p.created_at, p.updated_at,
               COUNT(ps.song_id) as song_count
        FROM playlists p
        LEFT JOIN playlist_songs ps ON ps.playlist_id = p.id
        WHERE p.profile_id = ?
        GROUP BY p.id
        ORDER BY p.created_at DESC
        "#,
    )
    .bind(profile_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let playlists = rows
        .into_iter()
        .map(|row| Playlist {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            cover_path: row.get("cover_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            song_count: row.get("song_count"),
        })
        .collect();

    Ok(playlists)
}

#[tauri::command]
pub async fn delete_playlist(pool: State<'_, SqlitePool>, playlist_id: i32) -> Result<(), String> {
    sqlx::query("DELETE FROM playlists WHERE id = ?")
        .bind(playlist_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_playlist(
    pool: State<'_, SqlitePool>,
    playlist_id: i32,
    name: String,
    cover_path: Option<String>,
) -> Result<(), String> {
    sqlx::query("UPDATE playlists SET name = ?, cover_path = ?, updated_at = strftime('%s','now') WHERE id = ?")
        .bind(name)
        .bind(cover_path)
        .bind(playlist_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn add_song_to_playlist(
    pool: State<'_, SqlitePool>,
    playlist_id: i32,
    song_id: i32,
) -> Result<(), String> {
    sqlx::query("INSERT OR IGNORE INTO playlist_songs (playlist_id, song_id, added_at) VALUES (?, ?, strftime('%s','now'))")
        .bind(playlist_id)
        .bind(song_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn remove_song_from_playlist(
    pool: State<'_, SqlitePool>,
    playlist_id: i32,
    song_id: i32,
) -> Result<(), String> {
    sqlx::query("DELETE FROM playlist_songs WHERE playlist_id = ? AND song_id = ?")
        .bind(playlist_id)
        .bind(song_id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_playlist_songs(
    pool: State<'_, SqlitePool>,
    playlist_id: i32,
    profile_id: i32,
) -> Result<Vec<TrackMetadata>, String> {
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
               EXISTS(SELECT 1 FROM favorites f WHERE f.song_id = s.id AND f.profile_id = s.profile_id) as is_favorite
        FROM songs s
        INNER JOIN playlist_songs ps ON ps.song_id = s.id
        WHERE ps.playlist_id = ? AND s.profile_id = ?
        ORDER BY ps.added_at ASC
        "#,
    )
    .bind(playlist_id)
    .bind(profile_id)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(row.get("is_favorite")),
        })
        .collect();

    Ok(tracks)
}

#[tauri::command]
pub async fn search_tracks(
    pool: State<'_, SqlitePool>,
    profile_id: i32,
    query: String,
) -> Result<Vec<TrackMetadata>, String> {
    let search_query = format!("%{}%", query);
    let rows = sqlx::query(
        r#"
        SELECT s.id, s.name, s.path, s.title, s.artist, s.album, s.duration, s.cover_path,
               EXISTS(SELECT 1 FROM favorites f WHERE f.song_id = s.id AND f.profile_id = s.profile_id) as is_favorite
        FROM songs s
        WHERE s.profile_id = ? AND (s.title LIKE ? OR s.artist LIKE ? OR s.album LIKE ?)
        ORDER BY s.title ASC
        LIMIT 50
        "#,
    )
    .bind(profile_id)
    .bind(&search_query)
    .bind(&search_query)
    .bind(&search_query)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;

    let tracks = rows
        .into_iter()
        .map(|row| TrackMetadata {
            id: row.get("id"),
            name: row.get("name"),
            path: row.get("path"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            duration: row.get("duration"),
            cover_path: row.get("cover_path"),
            is_favorite: Some(row.get("is_favorite")),
        })
        .collect();

    Ok(tracks)
}
