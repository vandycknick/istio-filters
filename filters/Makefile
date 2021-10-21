TARGET=""
# SERVICE=httpbin
# NAMESPACE=filters
SERVICE=istio-ingressgateway
NAMESPACE=istio-system

.PHONY: setup
setup:
	rustup target add wasm32-unknown-unknown

.PHONY: build
build:
	@find . -name Cargo.toml -not -path "*/wasm/module/*" | sed "s/\.\///" | xargs -n 1 dirname \
		| xargs -n 1 -I% -P1 bash -c "make TARGET=% build.wasm"

.PHONY: build.wasm
build.wasm:
	@mkdir -p bin
	@echo "Building ${TARGET}..."
	@cd ${TARGET}/wasm/module && \
		cargo build --target wasm32-unknown-unknown --release -p ${TARGET}-module && \
		cp target/wasm32-unknown-unknown/release/$$(echo ${TARGET} | sed 's/-/_/g')_module.wasm ../../../bin
	./utils/wabt-1.0.24/bin/wasm-strip bin/$$(echo ${TARGET} | sed 's/-/_/g')_module.wasm	


.PHONY: deploy
deploy:
	kubectl delete -n ${NAMESPACE} configmap wasm-modules 2>/dev/null || exit 0
	kubectl create -n ${NAMESPACE} configmap wasm-modules --from-file=./bin/

.PHONY: restart
restart:
	kubectl rollout restart -n ${NAMESPACE} deployment ${SERVICE}
	kubectl rollout status -n ${NAMESPACE} deployment ${SERVICE}
	$(MAKE) restart-istio

.PHONY: restart-istio
restart-istio:
	istioctl pc -n ${NAMESPACE} log $$(kubectl get -n ${NAMESPACE} pods -l app=${SERVICE} -o jsonpath={.items..metadata.name}) --level wasm:info