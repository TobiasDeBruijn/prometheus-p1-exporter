# Prometheus P1 Exporter
Prometheus exporter for Dutch electricity smart meters via the P1 port.

## Contents
- [Installation](#installation)
    - [Binary](#binary)
- [Metrics](#metrics)
- [Roadmap](#roadmap)
- [Developing](#developing)
    - [Crosscompiling](#crosscompiling)

## Installation
The following architectures are provided out of the box. If yours is not in here you'll have to compile it yourself.
- amd64
- aarch64 (arm64)
- arm32 (armhf)

### Binary
1. `apt install libudev-dev`
2. Download the correct binary for your architecture from [Releases](https://github.com/TheDutchMC/prometheus-p1-exporter/releases)
    >Note: If you use a Raspberry Pi model 0, 1 or 2 or run your Pi 3 with a 32-Bit OS you should download the `armhf` edition.
    For the Raspberry Pi model 4b+ you should use `arm64`
3. Add your user to the `dialout` group: `usermod -aG dialout <username>`
4. Create a systemd service by pasting the following snipping into `/etc/systemd/system/prometheus-p1-exporter.service`
```
[Unit]
Description=Prometheus P1 Exporter

[Service]
ExecStart=/usr/local/bin/prometheus-p1-exporter

[Install]
WantedBy=multi-user.target
```
>Note: This assumes you've placed the binary in `/usr/local/bin` and have named it `prometheus-p1-exporter`
5. Start the exporter
```
systemctl daemon reload && systemctl enable --now prometheus-p1-exporter
```

### Startup flags
`-p`: Set the port to listen on. Default: `9832`  
`-t`: Set the TTY device to use. Default: `/dev/ttyUSB0`  
`-i`: The interval in which to read from the TTY device in miliseconds. Default: `500`  

## Metrics
| Name                                       | Unit | Description                                                             |
|--------------------------------------------|------|-------------------------------------------------------------------------|
| actual_electricity_delivered_to_client     | kW   | Current electricity delivered to client in 1W resolution                |
| actual_electricity_received_from_client    | kW   | Current electricity received from client in 1W resolution               |
| electricity_delivered_to_client_tariff_1   | kWh  | Meter Reading electricity delivered to client (Tariff 1) in 0,001 kWh   |
| electricity_delivered_to_client_tariff_2   | kWh  | Meter Reading electricity delivered to client (Tariff 2) in 0,001 kWh   |
| electricity_received_from_client_tariff_1  | kWh  | Meter Reading electricity delivered from client (Tariff 1) in 0,001 kWh |
| electricity_received_from_client_tariff_2  | kWh  | Meter Reading electricity delivered from client (Tariff 2) in 0,001 kWh |

## Roadmap
- [ ] Add support for Gas meter readings

## Developing
You need the [Rust toolchain](https://www.rust-lang.org/learn/get-started), should be easy from there.
This exporter is based on [this document](https://www.netbeheernederland.nl/_upload/Files/Slimme_meter_15_a727fce1f1.pdf) from Netbeheer Nederland.

### Crosscompiling
1. Add the architectures, ``armhf`` and ``arm64``:
```
dpkg --add-architecture <arch>
```
2. Configure apt repositories
    1. In your `/etc/apt/sources.list`, add `[arch=amd64]` after every `deb`. E.g:
    ```
    deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ focal main restricted
    ```
    2. Paste into `/etc/apt/sources.list.d/arm.list`
    ```
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal main restricted
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates main restricted
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal universe
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates universe
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal multiverse
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates multiverse
    deb [arch=arm64,armhf] http://ports.ubuntu.com/ubuntu-ports focal-backports main restricted universe multiverse
    ```
3. Update apt repository cache
```
apt update
```
4. Install libudev for every arch
```
apt install -y libudev-dev:amd64 libudev-dev:arm64 libudev-dev:armhf
```
5. Rustup
```
rustup target add aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf
```
6. Paste the following into `~/.cargo/config`
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```
7. Compiling

**amd64**:
```
cargo build --target x86_64-unknown-linux-gnu --release
```
**arm64**:
```
PKG_CONFIG_SYSROOT_DIR=/usr/lib/aarch64-linux-gnu cargo build --target aarch64-unknown-linux-gnu --release
```
**armhf**:
```
PKG_CONFIG_SYSROOT_DIR=/usr/lib/arm-linux-gnueabihf cargo build --target arm-unknown-linux-gnueabihf --release
```

For convenience you can also use `Make`. `make amd64`, `make arm64` and `make arm64` respectively.
