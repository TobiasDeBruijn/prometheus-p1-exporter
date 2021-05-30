use actix_web::{HttpResponse, get};
use crate::metrics_storage::METRICS_PROM;
use prometheus::{TextEncoder, Encoder};

lazy_static! {
    static ref ENCODER: TextEncoder = TextEncoder::new();
}

#[get("/metrics")]
pub async fn metrics() -> HttpResponse {
    let guard = METRICS_PROM.lock().unwrap();
    let families = guard.gather();
    let mut buff = Vec::new();
    ENCODER.encode(&families, &mut buff).unwrap();

    HttpResponse::Ok().body(String::from_utf8(buff).unwrap())
}