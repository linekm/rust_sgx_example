default: release
.PHONY: default

debug:
	$(MAKE) -C hello-rust-enclave DEBUG=1
	cargo build
.PHONY: debug

release:
	$(MAKE) -C hello-rust-enclave
	cargo build --release
.PHONY: release

test:
	cargo test -- --nocapture
.PHONY: test

test-release:
	cargo test --release -- --nocapture
.PHONY: test-release

clean:
	-rm -rf target
	-$(MAKE) -C hello-rust-enclave clean
.PHONY: clean

clippy:
	cargo clippy --all-targets
.PHONY: clippy

check-deps:
	cargo upgrade --workspace --skip-compatible --dry-run
	$(MAKE) -C hello-rust-enclave check-deps
.PHONY: check-deps

update-deps:
	cargo update
	$(MAKE) -C hello-rust-enclave update-deps
.PHONY: update-deps

fmt:
	cargo fmt
	$(MAKE) -C hello-rust-enclave fmt
.PHONY: fmt

loc:
	tokei -e rust-sgx-sdk
.PHONY: loc
