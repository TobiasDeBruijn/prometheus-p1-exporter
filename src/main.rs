#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::channel;

use actix_web::{App, HttpServer};

mod serial;
mod metrics_storage;
mod metrics_endpoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut tty = "/dev/ttyUSB0";
    let mut scrape_interval = 500u64;
    let mut port = 9832;

    for mut i in 0..args.len() {
        if let Some(arg) = args.get(i) {
            match arg.as_str() {
                "-t" => {
                    i+=1;
                    if let Some(tty_local) = args.get(i) {
                        tty = tty_local
                    } else {
                        eprintln!("Argument '-t' requires a value.");
                        std::process::exit(1);
                    }
                },
                "-i" => {
                    i+=1;
                    if let Some(interval_local) = args.get(i) {
                        let interval_local = match interval_local.parse::<u64>() {
                            Ok(interval) => interval,
                            Err(err) => {
                                eprintln!("Failed to parse interval. Is it a number? {:?}", err);
                                std::process::exit(1);
                            }
                        };

                        scrape_interval = interval_local;
                    } else {
                        eprintln!("Argument '-i' requires a value.");
                        std::process::exit(1);
                    }
                },
                "-p" => {
                    i+=1;
                    if let Some(port_local) = args.get(i) {
                        let port_local = match port_local.parse::<u64>() {
                            Ok(port) => port,
                            Err(err) => {
                                eprintln!("Failed to parse port. Is it a number? {:?}", err);
                                std::process::exit(1);
                            }
                        };

                        port = port_local;
                    } else {
                        eprintln!("Argument '-p' requires a value.");
                        std::process::exit(1);
                    }
                }
                _ => {}
            }
        }
    }

    println!("Starting P1Reader on port {}", &port);
    println!("Reading {} for metrics.", tty);

    let (tx, rx) = channel();
    metrics_storage::storage_listener(rx);
    serial::read_port(tx, scrape_interval, tty);

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::new("%a %s $U %{User-Agent}i"))
            .service(metrics_endpoint::metrics)
    })
    .bind(&format!("0.0.0.0:{}", port))?
    .run()
    .await
}
