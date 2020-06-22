use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_server::{Server};

use crate::config;

async fn index(cfg: web::Data<config::Config>) -> impl Responder {
    println!("{}", cfg.port);
    "index"
}

async fn configjson(_req: HttpRequest) -> impl Responder {
    "config.json"
}

async fn rustup(_req: HttpRequest) -> impl Responder {
    "rustup"
}

pub fn new(cfg: config::Config) -> Server {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("listening addr {}", addr);

    let srv = HttpServer::new(move || {
        let cfg = cfg.clone();
        App::new()
            .data(cfg)
            .route("/", web::get().to(index))
            .service(
                web::scope("/crates.io-index")
                    .route("/config.json", web::get().to(configjson))
            )
            .service(
                web::scope("/rustup")
            )

    })
    .bind(addr);

    let srv = match srv {
        Ok(item) => item,
        Err(e) => panic!(e),
    };
    return srv.run();
}

