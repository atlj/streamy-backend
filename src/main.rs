use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use streamy::config::{self, Args};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Args = config::Args::parse();
    HttpServer::new(|| App::new().service(hello))
        .bind((config.address, config.port))?
        .run()
        .await
}
