use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::media::{self, Movie};

pub fn create_watcher(
    media_files_mutex: Arc<Mutex<Vec<Movie>>>,
    media_path: PathBuf,
) -> Result<notify::RecommendedWatcher, notify::Error> {
    notify::recommended_watcher(move |_| {
        if let Ok(mut media_files) = media_files_mutex.lock() {
            *media_files = media::scan_media(&media_path);
        }
    })
}
