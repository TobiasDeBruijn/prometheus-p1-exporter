use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    /// The TTY device the P1 meter is connected to
    #[clap(long, short, value_parser, default_value_t = String::from("/dev/ttyUSB0"))]
    pub tty: String,
    /// The scrape interval in miliseconds. This determines how often the P1 meter is polled
    #[clap(long, short, value_parser, default_value_t = 500)]
    pub scrape_interval: u64,
    /// The port to expose the Prometheus metrics on.
    #[clap(long, short, value_parser, default_value_t = 9832)]
    pub port: u16,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }
}