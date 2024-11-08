# Variables
GOOS=wasip1
GOARCH=wasm
GO_BINARY=go
OPTIMIZER=wasm-opt
WASM_TOOLS=wasm-tools
WASI_VIRT=wasi-virt
MAIN_WASM=main.wasm
OPT_WASM=main-opt.wasm
COMPONENT_WASM=my-component.wasm
VIRT_WASM=virtualized-component.wasm
ADAPT_MODULE=upstream/wasmtime/target/wasm32-unknown-unknown/release/wasi_snapshot_preview1.wasm

# Default target when running 'make'
.PHONY: build
build: compile opt componentize virtualize

# Compile target
.PHONY: compile
compile:
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO_BINARY) build -o $(MAIN_WASM) main.go

# Optimization target
.PHONY: opt
opt: $(MAIN_WASM)
	$(OPTIMIZER) $(MAIN_WASM) --enable-bulk-memory -o $(OPT_WASM) -O1

# Componentize target
.PHONY: componentize
componentize: $(OPT_WASM)
	$(WASM_TOOLS) component new $(OPT_WASM) -o $(COMPONENT_WASM) --adapt $(ADAPT_MODULE)

# Virtualization target
.PHONY: virtualize
virtualize: $(OPT_WASM)
	$(WASI_VIRT) --allow-stdio --out $(VIRT_WASM) my-component.wasm
