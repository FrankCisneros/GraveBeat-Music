use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use rayon::prelude::*;
use serde::Serialize;
use std::fs;
use std::time::Instant;
use tauri::{command, AppHandle, Emitter, Manager};
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
pub struct ScanProgress {
    pub current: u32,
    pub total: u32,
}

#[derive(Serialize, serde::Deserialize)]
pub struct TrackMetadata {
    pub id: Option<i32>,
    pub name: String,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<i64>,
    pub cover_path: Option<String>,
    pub is_favorite: Option<bool>,
}

#[command]
pub fn scan_folder(app: AppHandle, path: String) -> Vec<TrackMetadata> {
    const EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

    log::info!("[{path}] Starting folder scan...");
    let start = Instant::now();

    // Resolve cache directory once
    let cache_dir = app.path().app_cache_dir().ok();

    let files: Vec<_> = WalkDir::new(&path)
        .max_depth(4)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    let total_files = files.len() as u32;
    let processed = std::sync::atomic::AtomicU32::new(0);

    let tracks: Vec<TrackMetadata> = files
        .into_par_iter()
        .filter_map(|entry| {
            let p = entry.path();
            let ext_ok = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| EXTENSIONS.iter().any(|&x| x.eq_ignore_ascii_case(s)))
                .unwrap_or(false);

            let processed_count = processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if ext_ok || processed_count % 10 == 0 || processed_count == total_files {
                let _ = app.emit(
                    "scan-progress",
                    ScanProgress {
                        current: processed_count,
                        total: total_files,
                    },
                );
            }

            if !ext_ok {
                return None;
            }

            let name = p
                .file_name()
                .map(|os| os.to_string_lossy().into_owned())
                .unwrap_or_default();

            let full_path = p.to_string_lossy().into_owned();

            // --- Leer el archivo completo (tags + propiedades) ---
            let tagged_file = match Probe::open(p).and_then(|prb| prb.read()) {
                Ok(f) => f,
                Err(err) => {
                    log::warn!("Failed to read file {:?}: {}", p, err);
                    return None;
                }
            };

            // Obtener duración desde las propiedades de audio (Duration)
            let duration_secs = {
                let dur = tagged_file.properties().duration();
                // Si quieres más precisión podrías usar dur.as_secs_f64() o dur.as_millis()
                Some(dur.as_secs())
            };

            // Obtener el tag (title/artist/album) y la carátula
            let tag_opt = tagged_file.primary_tag();

            let (title, artist, album, cover_path) = tag_opt
                .map(|tag| {
                    let title = tag.title().map(String::from);
                    let artist = tag.artist().map(String::from);
                    let album = tag.album().map(String::from);
                    let safe_album_name =
                        album.clone().unwrap_or_else(|| "Desconocido".to_string());

                    let cover_path =
                        if let (Some(tag), Some(base_cache_dir)) = (tag_opt, &cache_dir) {
                            tag.pictures().first().and_then(|pic| {
                                let cover_dir = base_cache_dir.join("covers");
                                if let Err(_) = fs::create_dir_all(&cover_dir) {
                                    return None;
                                }

                                let file_name = format!("{:x}.jpg", md5::compute(&safe_album_name));
                                let full_cover_path = cover_dir.join(file_name);
                                let pic_data = pic.data();

                                let mut should_write = true;
                                if let Ok(metadata) = fs::metadata(&full_cover_path) {
                                    if (metadata.len() as usize) >= pic_data.len() {
                                        should_write = false;
                                    }
                                }

                                if should_write {
                                    if let Err(_) = fs::write(&full_cover_path, pic_data) {
                                        return None;
                                    }
                                }

                                Some(full_cover_path.to_string_lossy().into_owned())
                            })
                        } else {
                            None
                        };

                    (title, artist, album, cover_path)
                })
                .unwrap_or((None, None, None, None));

            Some(TrackMetadata {
                id: None,
                name,
                path: full_path,
                title,
                artist,
                album: album.or(Some("Desconocido".to_string())),
                duration: duration_secs.map(|d| d as i64),
                cover_path,
                is_favorite: Some(false),
            })
        })
        .collect();

    let elapsed = start.elapsed();
    log::info!(
        "[{}] Scan complete: found {} tracks in {:.2?}",
        path,
        tracks.len(),
        elapsed
    );

    tracks
}

#[command]
pub fn get_lyrics(path: String) -> Option<String> {
    // Priority 1: Check sidecar .lrc file
    let lrc_path = std::path::Path::new(&path).with_extension("lrc");
    if let Ok(lrc_content) = std::fs::read_to_string(&lrc_path) {
        return Some(lrc_content);
    }

    // Priority 2: Check ID3/embed tags
    use lofty::tag::ItemKey;
    use lofty::tag::ItemValue;

    let probe = Probe::open(&path).ok()?;
    let tagged_file = probe.read().ok()?;

    for tag in tagged_file.tags() {
        for item in tag.items() {
            if item.key() == ItemKey::Lyrics {
                if let ItemValue::Text(text) = item.value() {
                    return Some(text.to_string());
                }
            }
        }
    }

    None
}

#[command]
pub fn save_lyrics(track_path: String, lyrics: String) -> Result<(), String> {
    let path = std::path::Path::new(&track_path);
    let lrc_path = path.with_extension("lrc");
    std::fs::write(&lrc_path, lyrics).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn is_folder_modified(path: String, last_scan: i64) -> bool {
    for entry in WalkDir::new(&path).into_iter().filter_map(Result::ok) {
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                if let Ok(dur) = modified.duration_since(std::time::UNIX_EPOCH) {
                    if dur.as_secs() as i64 > last_scan {
                        return true;
                    }
                }
            }
        }
    }
    false
}
