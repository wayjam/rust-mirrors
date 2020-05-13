use structopt::StructOpt;
use std::path::PathBuf;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn index(_req: HttpRequest) -> impl Responder {
    "index"
}

#[derive(StructOpt)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Config file
    #[structopt(short, long, parse(from_os_str))]
    config: PathBuf,

    /// Port of web server to use
    #[structopt(short, long, default_value = "8080")]
    port: u16,

    /// Host of web server to use
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let addr = format!("{}:{}", opt.host, opt.port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))

    })
    .bind(addr)?
    .run()
    .await
}

