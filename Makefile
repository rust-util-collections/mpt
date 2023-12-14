all: fmt lint

release: build_release

build:
	cargo build --bins

build_release:
	cargo build --release --bins

lint:
	cargo clippy --all-features
	cargo check --tests
	cargo check --benches

test:
	cargo test -- --test-threads=1

test_release:
	cargo test --release -- --test-threads=1

bench:
	cargo bench

fmt:
	cargo fmt

fmtall:
	bash tools/fmt.sh

clean:
	cargo clean

deep_clean: clean
	git stash
	git clean -fdx

update: gitmod
	rustup update stable
	cargo update

doc:
	cargo doc --open

gitmod:
	git submodule update --init --recursive
