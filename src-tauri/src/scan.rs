use lofty::file::{ TaggedFileExt, AudioFile };
use lofty::probe::Probe;
use lofty::tag::{Accessor, TagExt};
use rayon::prelude::*;
use serde::Serialize;
use std::time::Instant;
use tauri::command;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct TrackMetadata {
    pub name: String,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<u64>,           // segundos
    pub cover_data_url: Option<String>,
}

#[command]
pub fn scan_folder(path: String) -> Vec<TrackMetadata> {
    const EXTENSIONS: &[&str] = &["mp3", "flac", "wav", "ogg", "m4a"];

    log::info!("[{path}] Starting folder scan...");
    let start = Instant::now();

    let tracks: Vec<TrackMetadata> = WalkDir::new(&path)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .par_bridge()
        .filter_map(|entry| {
            let p = entry.path();
            let ext_ok = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| EXTENSIONS.iter().any(|&x| x.eq_ignore_ascii_case(s)))
                .unwrap_or(false);
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

            let (title, artist, album, cover_data_url) = tag_opt.map(|tag| {
                let title = tag.title().map(String::from);
                let artist = tag.artist().map(String::from);
                let album = tag.album().map(String::from);

                let cover_data_url = tag.pictures().first().map(|pic| {
                    let mime = pic
                        .mime_type()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "application/octet-stream".into());
                    let b64 = base64::encode(pic.data());
                    format!("data:{mime};base64,{b64}")
                });

                (title, artist, album, cover_data_url)
            }).unwrap_or((None, None, None, None));

            Some(TrackMetadata {
                name,
                path: full_path,
                title,
                artist,
                album,
                duration: duration_secs,
                cover_data_url,
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
