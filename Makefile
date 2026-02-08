dev:
	@DEBUG=1 cargo run -p kiosk-ui
check:
	@cargo check
build:
	@cargo build --release
clippy:
	@cargo clippy
