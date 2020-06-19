use clap::{value_t, App, Arg, crate_version};

mod config;
mod server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Rust Mirror")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listen_host")
                .short("h")
                .long("host")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listen_port")
                .short("p")
                .long("port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .takes_value(false)
                .help("activate debug mode"),
        )
        .version(crate_version!())
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.json");

    let cfg = config::Config::from_file(config_path);
    cfg.set_debug(matches.is_present("debug"));
        // .set_host(matches
        // .value_of("listen_host")
        // .unwrap_or("127.0.0.1"))
        // .set_port(value_t!(matches, "listen_port", u16).unwrap_or(8080));

    println!("Using Config:{}", config_path);

    let srv = server::new(&cfg);

    srv.await
}
