use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// This scans the media path and returns a list of files that can be
/// served.
pub fn scan_media(media_path: &PathBuf) -> Vec<String> {
    WalkDir::new(media_path)
        .into_iter()
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();

            filter_media(path)?;

            path.strip_prefix(media_path)
                .ok()?
                .to_str()
                .map(|s| s.to_string())
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
