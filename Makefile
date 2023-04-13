fmt:
	@cargo fmt

run: fmt
	@cd static/databricks-templates && mkdocs build
	@cargo run --bin echo-server

build: fmt
	@cargo build --release

run-release: build
	@./target/release/echo-server

docker:
	@BUILDKIT_PROGRESS=plain docker build -t echo-server .

docker-run:
	docker run -it 3000:3000 echo-server