use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use streamy::{
    config::{self, Args},
    media::scan_media,
};

struct AppState {
    config: Args,
}

type AppData = web::Data<AppState>;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/list-media")]
async fn list_media(state: AppData) -> impl Responder {
    let media_files = scan_media(&state.config.media_path);
    HttpResponse::Ok().json(media_files)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Args::parse();
    let Args { address, port, .. } = config.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                config: config.clone(),
            }))
            .service(Files::new("/media", &config.media_path).show_files_listing())
            .service(hello)
            .service(list_media)
    });

    server.bind((address, port))?.run().await
}
