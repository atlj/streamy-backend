use serde::Serialize;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct Movie {
    pub imdb_id: String,
    pub media_path: String,
}

/// This scans the media path and returns a list of files that can be
/// served.
pub fn scan_media(media_path: &PathBuf) -> Vec<Movie> {
    WalkDir::new(media_path)
        .into_iter()
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();

            filter_media(path)?;

            let imdb_id = path.parent()?.file_name()?.to_str()?.to_string();
            let media_path = path.strip_prefix(media_path).ok()?.to_str()?.to_string();

            Some(Movie {
                imdb_id,
                media_path,
            })
        })
        .collect()
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
