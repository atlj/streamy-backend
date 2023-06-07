use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Serialize)]
pub struct MediaItem {
    pub media_path: String,
    pub movie: Movie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub image_url: String,
}

/// This scans the media path and returns a list of files that can be
/// served.
// TODO: Remove sphagetti code
pub fn scan_media(media_path: &PathBuf) -> Vec<MediaItem> {
    WalkDir::new(media_path)
        .into_iter()
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();

            filter_media(path)?;
            Some(file)
        })
        .filter_map(generate_media_item)
        .filter_map(|(movie, media_dir)| {
            Some(MediaItem {
                movie,
                media_path: media_dir
                    .path()
                    .strip_prefix(media_path)
                    .ok()?
                    .to_str()?
                    .to_string(),
            })
        })
        .collect()
}

fn generate_media_item(media_dir: DirEntry) -> Option<(Movie, DirEntry)> {
    let media_path = media_dir.path();
    let parent = media_path.parent()?;
    let meta_data_path = parent.join("meta.json");
    let meta_data_serialized = fs::read_to_string(meta_data_path).ok()?;

    Some((serde_json::from_str(&meta_data_serialized).ok()?, media_dir))
}

/// TODO: Get this from the config
const MEDIA_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];

/// This function filters out directories and non-media files.
fn filter_media(path: &Path) -> Option<()> {
    if let Some(ext) = path.extension()?.to_str() {
        if MEDIA_EXTENSIONS.contains(&ext) {
            return Some(());
        }
    }

    None
}
