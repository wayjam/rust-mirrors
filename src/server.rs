use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse, Error};
use actix_server::{Server};
use actix_web::client::Client;
use url::{Url, ParseError};

use crate::config;

async fn index(cfg: web::Data<config::Config>) -> impl Responder {
    println!("{}", cfg.port);
    "index"
}

async fn configjson(_req: HttpRequest) -> impl Responder {
    "config.json"
}

async fn cratesio() -> impl Responder {
    "crateio"
}

async fn rustup(_req: HttpRequest) -> impl Responder {
    "rustup"
}

async fn proxy_crate_index(
    req: HttpRequest,
    body: web::Bytes,
    cfg: web::Data<config::Config>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let new_url = Url::parse("http://127.0.0.1:80");
    return Ok(proxy(req, body, new_url, client).await);
}


async fn proxy(
    req: HttpRequest,
    body: web::Bytes,
    url: Result<url::Url,url::ParseError>,
    client: web::Data<Client>,
) -> HttpResponse {
    let mut url = url.unwrap();
    url.set_path(req.uri().path());
    url.set_query(req.uri().query());

    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());
    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    return client_resp.body(res.body().await?);
    // Ok(client_resp.body(res.body().await?))
    // Ok(client_resp.streaming(res.body()))
}


pub fn new(cfg: config::Config) -> Server {
    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("listening addr {}", addr);

    let srv = HttpServer::new(move || {
        let cfg = cfg.clone();
        App::new()
            .data(Client::new())
            .data(cfg)
            .route("/", web::get().to(index))
            .service(
                web::scope("/crates.io-index")
                    .route("/config.json", web::get().to(configjson))
                    .route("/", web::get().to(proxy_crate_index))
            )
            .service(
                web::scope("/crates.io")
                    .route("/", web::get().to(cratesio)
            ))
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

