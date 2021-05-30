all: release
.PHONY: amd64 arm64 armhf release clean

RUST_SOURCE_FILES := $(shell find src/ -type f)

amd64: target/x86_64-unknown-linux-gnu/release/p1_reader
arm64: target/aarch64-unknown-linux-gnu/release/p1_reader
armhf: target/arm-unknown-linux-gnueabihf/release/p1_reader

target/x86_64-unknown-linux-gnu/release/p1_reader: ${RUST_SOURCE_FILES}
	cargo build --target x86_64-unknown-linux-gnu --release

target/aarch64-unknown-linux-gnu/release/p1_reader: ${RUST_SOURCE_FILES}
	PKG_CONFIG_SYSROOT_DIR=/usr/lib/aarch64-linux-gnu cargo build --target aarch64-unknown-linux-gnu --release

target/arm-unknown-linux-gnueabihf/release/p1_reader: ${RUST_SOURCE_FILES}
	PKG_CONFIG_SYSROOT_DIR=/usr/lib/arm-linux-gnueabihf cargo build --target arm-unknown-linux-gnueabihf --release

release: amd64 arm64 armhf
	mkdir -p release
	cp target/x86_64-unknown-linux-gnu/release/p1_reader release/prometheus-p1-exporter-amd64
	cp target/aarch64-unknown-linux-gnu/release/p1_reader release/prometheus-p1-exporter-arm64
	cp target/arm-unknown-linux-gnueabihf/release/p1_reader release/prometheus-p1-exporter-armhf

clean:
	rm -rf release target