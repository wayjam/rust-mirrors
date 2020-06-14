use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_server::{Server};

use crate::config;

async fn index(_req: HttpRequest) -> impl Responder {
    "index"
}

async fn configjson(_req: HttpRequest) -> impl Responder {
    "config.json"
}


pub fn new(cfg: &config::Config) -> Server {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    let srv = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/config.json", web::get().to(configjson))

    })
    .bind(addr);

    let srv = match srv {
        Ok(item) => item,
        Err(e) => panic!(e),
    };
    return srv.run();
}

