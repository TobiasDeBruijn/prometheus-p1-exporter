use std::sync::mpsc::Sender;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct PortData {
    ///True if any value in the struct has been changed from it's default.
    set: bool,

    ///Meter reading electricity delivered to client Tariff 1 in 0.001 kWh
    pub e_delivered_to_t1: f64,

    ///Meter reading electricity delivered to client Tariff 2 in 0.001 kWh
    pub e_delivered_to_t2: f64,

    ///Meter reading electricity delivered from client Tariff 1 in 0.001 kwH
    pub e_delivered_from_t1: f64,

    ///Meter reading electricity delivered from client Tariff 2 in 0.001 kwH
    pub e_delivered_from_t2: f64,

    ///Actual electricity power delivered (+P) in 1 Watt resolution
    pub actual_e_delivered: f64,

    ///Actual electricity power received (-P) in 1 Watt resolution
    pub actual_e_received: f64
}

impl Default for PortData {
    fn default() -> Self {
        Self {
            set: false,
            e_delivered_from_t1: 0f64,
            e_delivered_from_t2: 0f64,
            e_delivered_to_t1: 0f64,
            e_delivered_to_t2: 0f64,
            actual_e_delivered: 0f64,
            actual_e_received: 0f64
        }
    }
}

pub fn read_port(tx: Sender<PortData>, scrape_interval: u64, tty: &str) {
    let mut port = serialport::new(tty, 115_200)
        .timeout(std::time::Duration::from_millis(500))
        .open()
        .expect("Failed to open serial port /dev/ttyUSB0.");

    let data_line_regex = Regex::new(r#"\d-\d"#).unwrap();
    let e_delivered_to_t1_regex = Regex::new(r#"(1-0:1\.8\.1)\((.*)\*kWh\)"#).unwrap();
    let e_delivered_to_t2_regex = Regex::new(r#"(1-0:1\.8\.2)\((.*)\*kWh\)"#).unwrap();
    let e_delivered_from_t1_regex = Regex::new(r#"(1-0:2\.8\.1)\((.*)\*kWh\)"#).unwrap();
    let e_delivered_from_t2_regex = Regex::new(r#"(1-0:2\.8\.2)\((.*)\*kWh\)"#).unwrap();
    let actual_e_delivered = Regex::new(r#"(1-0:1\.7\.0)\((.*)\*kW\)"#).unwrap();
    let actual_e_received =  Regex::new(r#"(1-0:2\.7\.0)\((.*)\*kW\)"#).unwrap();

    std::thread::spawn(move || {
        loop {
            let mut buf = String::new();
            let _ = port.read_to_string(&mut buf);

            //The P1 port on the meter only gives data every now and then, if it's empty we can just skip a cycle.
            if buf.is_empty() {
                continue;
            }

            let mut portdata = PortData::default();

            for line in buf.lines() {
                //Empty line, we don't care, move on
                if line.is_empty() {
                    continue;
                }

                //If this doesn't match, it's likely not a valid data line (noise)
                if !data_line_regex.is_match(line) {
                    continue;
                }

                if let Some(v) = check_line(line, &e_delivered_to_t1_regex) {
                    portdata.e_delivered_to_t1 = v;
                    portdata.set = true;
                }

                if let Some(v) = check_line(line, &e_delivered_to_t2_regex) {
                    portdata.e_delivered_to_t2 = v;
                    portdata.set = true;
                }

                if let Some(v) = check_line(line, &e_delivered_from_t1_regex) {
                    portdata.e_delivered_from_t1 = v;
                    portdata.set = true;
                }

                if let Some(v) = check_line(line, &e_delivered_from_t2_regex) {
                    portdata.e_delivered_from_t2 = v;
                    portdata.set = true;
                }

                if let Some(v) = check_line(line, &actual_e_delivered) {
                    portdata.actual_e_delivered = v;
                    portdata.set = true;
                }

                if let Some(v) = check_line(line, &actual_e_received) {
                    portdata.actual_e_received = v;
                    portdata.set = true;
                }
            }

            if !portdata.set {
                continue;
            }

            tx.send(portdata).expect("Failed to send PortData");
            std::thread::sleep(std::time::Duration::from_millis(scrape_interval));
        }
    });
}

fn check_line(line: &str, regex: &Regex) -> Option<f64> {
    if regex.is_match(line) {
        let regex_match = regex.captures(line);
        if let Some(capture) = regex_match {
            if let Some(v) = capture.get(2) {
                let value = v.as_str().parse::<f64>().expect(&format!("Failed to parse line: {}", line));
                return Some(value);
            }
        }
    }
    None
}