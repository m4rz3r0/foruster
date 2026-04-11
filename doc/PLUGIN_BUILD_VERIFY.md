# Building and verifying WASM plugins

This checklist complements [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) and [PLUGIN_SDK.md](PLUGIN_SDK.md). It describes how to build the workspace, compile sample extensions, and run plugin-related tests without machine-specific paths.

## Prerequisites

- Rust toolchain matching the workspace (`rust-version` in the root `Cargo.toml`).
- WebAssembly target:

```bash
rustup target add wasm32-wasip1
```

## 1. Build the main workspace

From the repository root:

```bash
cargo build --workspace
```

Optional static analysis:

```bash
cargo clippy --workspace -- -W clippy::all
```

If dependency versions drift (e.g. Wasmtime, `ort`), align the host and workspace dependency pins with the supported SDK release.

## 2. Build sample plugins

Either use the Makefile (POSIX shell):

```bash
make plugins
```

Or run `./scripts/build-plugins.sh` (same as `make plugins`) or `./scripts/build-plugins.fish` on Fish. Built `.wasm` files are copied under `plugins/`.

To build one example manually:

```bash
cd examples/simple-detector
cargo build --target wasm32-wasip1 --release
```

Copy the resulting `.wasm` next to the Foruster binary or into `plugins/` as described in [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md).

## 3. Inspect the module (optional)

With [WABT](https://github.com/WebAssembly/wabt) (`wasm-objdump`):

```bash
wasm-objdump -x plugins/simple_detector.wasm | grep -A 20 Export
wasm-objdump -x plugins/simple_detector.wasm | grep -A 10 Import
```

Exports should include the symbols required by the host (see [PLUGIN_SDK.md](PLUGIN_SDK.md)).

## 4. Tests

Workspace tests:

```bash
cargo test --workspace
```

Plugin integration tests ship with the **full host sources** (not this documentation mirror). If you have access to that tree, run the integration tests described there.

## 5. Troubleshooting

**Cannot find crate `foruster_plugin_sdk`**

Build the SDK from `plugin-sdk/`:

```bash
cargo check --manifest-path plugin-sdk/Cargo.toml
```

Or use `bash scripts/check-plugin-sdk.sh`.

**Wasmtime version mismatch**

Align Wasmtime-related dependency versions with the version set used for the shipping host build.

**Plugin not discovered**

Ensure `.wasm` files are in `plugins/` relative to the working directory or layout described in [FORENSIC_POLICY.md](FORENSIC_POLICY.md).

**ONNX / `ort` issues**

See [STATIC_BUILDS.md](STATIC_BUILDS.md) and the root `Cargo.toml` for how inference dependencies are configured.
