use std::sync::mpsc::Receiver;
use crate::serial::PortData;
use std::sync::{Mutex, Arc};
use prometheus::{Registry, Gauge};

lazy_static! {
    pub static ref METRICS_PROM: Arc<Mutex<Registry>> = Arc::new(Mutex::new(Registry::new()));
}

pub fn storage_listener(rx: Receiver<PortData>) {
    std::thread::spawn(move || {
        let gauge_e_delivered_to_t1 = Gauge::new("electricity_delivered_to_client_tariff_1", "Meter Reading electricity delivered to client (Tariff 1) in 0,001 kWh").unwrap();
        let gauge_e_delivered_to_t2 = Gauge::new("electricity_delivered_to_client_tariff_2", "Meter Reading electricity delivered to client (Tariff 2) in 0,001 kWh").unwrap();
        let gauge_e_delivered_from_t1 = Gauge::new("electricity_received_from_client_tariff_1", "Meter Reading electricity delivered from client (Tariff 1) in 0,001 kWh").unwrap();
        let gauge_e_delivered_from_t2 = Gauge::new("electricity_received_from_client_tariff_2", "Meter Reading electricity delivered from client (Tariff 1) in 0,001 kWh").unwrap();
        let gauge_actual_e_delivered = Gauge::new("actual_electricity_delivered_to_client", "Actual electricity power delivered (+P) in 1 Watt resolution").unwrap();
        let gauge_actual_e_received = Gauge::new("actual_electricity_received_from_client", "Actual electricity power received (-P) in 1 Watt resolution").unwrap();

        {
            let guard = METRICS_PROM.lock().expect("Failed to lock prometheus metrics storage");
            guard.register(Box::new(gauge_e_delivered_to_t1.clone())).unwrap();
            guard.register(Box::new(gauge_e_delivered_to_t2.clone())).unwrap();
            guard.register(Box::new(gauge_e_delivered_from_t1.clone())).unwrap();
            guard.register(Box::new(gauge_e_delivered_from_t2.clone())).unwrap();
            guard.register(Box::new(gauge_actual_e_delivered.clone())).unwrap();
            guard.register(Box::new(gauge_actual_e_received.clone())).unwrap();
        }

        loop {
            let recv = match rx.recv() {
                Ok(recv) => recv,
                Err(err) => {
                    panic!("Failed to receive data: {:?}", err)
                }
            };

            gauge_e_delivered_to_t1.set(recv.e_delivered_to_t1);
            gauge_e_delivered_to_t2.set(recv.e_delivered_to_t2);
            gauge_e_delivered_from_t1.set(recv.e_delivered_from_t1);
            gauge_e_delivered_from_t2.set(recv.e_delivered_from_t2);
            gauge_actual_e_delivered.set(recv.actual_e_delivered);
            gauge_actual_e_received.set(recv.actual_e_received);
        }
    });
}

