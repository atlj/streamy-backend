use std::path::PathBuf;
use walkdir::WalkDir;

/// This module actively scans for the media path and returns a list of files
/// that can be served.
///
/// It
///

#[derive(Debug)]
pub struct MediaList {
    pub files: Vec<String>,
}

impl MediaList {
    pub fn new(media_path: &PathBuf) -> Self {
        MediaList {
            files: scan_media(media_path),
        }
    }

    pub fn on_update(&mut self) {
        // TODO
    }
}

/// This scans the media path and returns a list of files that can be
/// served.
/// TODO: Refactor
fn scan_media(media_path: &PathBuf) -> Vec<String> {
    WalkDir::new(media_path)
        .into_iter()
        .filter_map(|file| {
            let file = file.ok()?;
            if file.file_type().is_dir() {
                return None;
            }
            if let Some(ext) = file.path().extension() {
                if ext != "mp4" && ext != "mov" && ext != "mkv" && ext != "avi" {
                    return None;
                }
            } else {
                return None;
            }
            let path = file.path();
            let path = path.to_str()?;
            let mut path = path.to_string();
            // Remove the media path from the path
            path.replace_range(0..media_path.to_str()?.len(), "");
            Some(path)
        })
        .collect()
}
