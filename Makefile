dev:
	@DEBUG=1 cargo run -p ui
check:
	@cargo check
build:
	@cargo build --release
clippy:
	@cargo clippy
