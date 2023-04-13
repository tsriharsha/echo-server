fmt:
	@cargo fmt

run: fmt
	@cargo run --bin echo-server

build: fmt
	@cargo build --release

run-release: build
	@./target/release/echo-server

docker:
	@BUILDKIT_PROGRESS=plain docker build -t echo-server .

docker-run:
	docker run -it 3000:3000 echo-server