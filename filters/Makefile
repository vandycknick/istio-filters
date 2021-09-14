.PHONY: build
build:
	cd jwt-claims-header/wasm/module && \
		cargo build --target wasm32-unknown-unknown --release -p jwt-claims-header-module && \
		cp target/wasm32-unknown-unknown/release/jwt_claims_header_module.wasm ../../../bin
	./utils/wabt-1.0.24/bin/wasm-strip bin/jwt_claims_header_module.wasm

.PHONY: build-2
build-2:
	cd jwt-from-cookie/wasm/module && \
		cargo build --target wasm32-unknown-unknown --release -p jwt-from-cookie-module && \
		cp target/wasm32-unknown-unknown/release/jwt_from_cookie_module.wasm ../../../bin
	./utils/wabt-1.0.24/bin/wasm-strip bin/jwt_from_cookie_module.wasm
