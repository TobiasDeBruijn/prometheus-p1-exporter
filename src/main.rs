#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::channel;

use actix_web::{App, HttpServer, web};
use crate::args::Args;

mod serial;
mod metrics_storage;
mod metrics_endpoint;
mod args;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::new();

    println!("Starting P1Reader on port {}", args.port);
    println!("Reading {} for metrics.", args.tty);

    let (tx, rx) = channel();
    metrics_storage::storage_listener(rx);
    serial::read_port(tx, args.scrape_interval, &args.tty);

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::new("%a %s $U %{User-Agent}i"))
            .route("/metrics", web::get().to(metrics_endpoint::metrics))
    })
    .bind(&format!("0.0.0.0:{}", args.port))?
    .run()
    .await
}
