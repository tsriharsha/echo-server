fmt:
	@cargo fmt

run: fmt
	@cargo run --bin echo-server

build: fmt
	@cargo build --release

run-release: build
	@./target/release/echo-server

docker:
	docker build -t echo-server .