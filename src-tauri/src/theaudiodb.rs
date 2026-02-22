use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::io::Write;
use tauri::{command, AppHandle, Manager};

#[command]
pub async fn fetch_artist_image(
    app: AppHandle,
    artist_name: String,
) -> Result<Option<String>, String> {
    let client = Client::new();
    // API Key "2" is the free test key for TheAudioDB
    let url = format!(
        "https://www.theaudiodb.com/api/v1/json/2/search.php?s={}",
        urlencoding::encode(&artist_name)
    );

    let res = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Ok(None);
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;

    // Extract image URL (strArtistThumb)
    let image_url = json["artists"]
        .as_array()
        .and_then(|items| items.first())
        .and_then(|artist| {
            // Try thumb first, then fanart, then clearart
            artist["strArtistThumb"]
                .as_str()
                .or(artist["strArtistFanart"].as_str())
                .or(artist["strArtistClearart"].as_str())
        })
        .map(|s| s.to_string());

    if let Some(url) = image_url {
        // Download the image
        let image_bytes = client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .bytes()
            .await
            .map_err(|e| e.to_string())?;

        // Save to local cache "artists" folder
        let cache_dir = app.path().app_cache_dir().map_err(|e| e.to_string())?;
        let artists_dir = cache_dir.join("artists");

        if !artists_dir.exists() {
            fs::create_dir_all(&artists_dir).map_err(|e| e.to_string())?;
        }

        let file_name = format!("{:x}.jpg", md5::compute(artist_name.as_bytes()));
        let file_path = artists_dir.join(&file_name);

        // Blocking file I/O is technically "blocking" but for small files usually acceptable in Tauri commands
        // unless extremely heavy. For absolute correctness we could use tokio::fs, but std::fs is fine here
        // to avoid rewriting everything. The main crash was reqwest::blocking inside async.
        let mut file = fs::File::create(&file_path).map_err(|e| e.to_string())?;
        file.write_all(&image_bytes).map_err(|e| e.to_string())?;

        return Ok(Some(file_path.to_string_lossy().into_owned()));
    }

    Ok(None)
}
