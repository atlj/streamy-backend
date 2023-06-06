use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use streamy::{
    config::{self, Args},
    media::MediaList,
};

// This struct represents state
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
    let media_list = MediaList::new(&state.config.media_path);
    HttpResponse::Ok().json(media_list.files)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Args::parse();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                // TODO: refactor
                config: config::Args::parse(),
            }))
            .service(Files::new("/media", config::Args::parse().media_path).show_files_listing())
            .service(hello)
            .service(list_media)
    })
    .bind((config.address, config.port))?
    .run()
    .await
}
