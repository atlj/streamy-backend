use std::sync::{Arc, Mutex};

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use notify::Watcher;
use streamy::{
    config::{self, Args},
    media::scan_media,
};

struct AppState {
    media_files: Arc<Mutex<Vec<String>>>,
}
type AppData = web::Data<AppState>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/list-media")]
async fn list_media(data: AppData) -> impl Responder {
    if let Ok(media_files) = data.media_files.lock() {
        return HttpResponse::Ok().json(media_files.clone());
    }
    return HttpResponse::InternalServerError().finish();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Args::parse();
    let Args {
        address,
        port,
        media_path,
    } = config.clone();

    let Args {
        media_path: media_path_watcher,
        ..
    } = config.clone();

    let media_files = Arc::new(Mutex::new(scan_media(&media_path)));
    let media_files_watcher = media_files.clone();

    let mut watcher = notify::recommended_watcher(move |_| {
        if let Ok(mut media_files) = media_files_watcher.lock() {
            *media_files = scan_media(&media_path_watcher);
        }
    })
    .unwrap();
    watcher
        .watch(&media_path, notify::RecursiveMode::Recursive)
        .unwrap();

    let server = HttpServer::new(move || {
        let files_service = Files::new("/media", &config.media_path).show_files_listing();
        let state = web::Data::new(AppState {
            media_files: media_files.clone(),
        });

        App::new()
            .app_data(state)
            .service(files_service)
            .service(hello)
            .service(list_media)
    });

    server.bind((address, port))?.run().await
}
