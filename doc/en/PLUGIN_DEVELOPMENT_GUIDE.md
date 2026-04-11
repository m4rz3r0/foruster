# Foruster plugin development guide

Hands-on guide for building **WASM plugins** for Foruster. For the normative SDK contract (types, ABI, host imports), see **[PLUGIN_SDK.md](PLUGIN_SDK.md)**.

**Language:** [Español →](../es/PLUGIN_DEVELOPMENT_GUIDE.md)

## Contents

1. [Environment setup](#environment-setup)
2. [Your first plugin](#your-first-plugin)
3. [SDK API overview](#sdk-api-overview)
4. [Host functions](#host-functions)
5. [Build and test](#build-and-test)
6. [Good practices](#good-practices)
7. [Debugging](#debugging)

---

## Environment setup

### Requirements

- Rust **1.87+** with target `wasm32-wasip1`
- **foruster-plugin-sdk** (path or published crate as documented for your tree)

### Installation

```bash
rustup target add wasm32-wasip1
cargo new --lib my-detector
cd my-detector
```

### `Cargo.toml`

```toml
[package]
name = "my-detector"
version = "1.0.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
foruster-plugin-sdk = { path = "../plugin-sdk" }

[profile.release]
opt-level = "z"
lto = true
strip = true
```

---

## Your first plugin

### Minimum structure

Every plugin exports **`plugin_metadata`** (identity) and **`plugin_analyze`** (logic). The **`declare_plugin!`** macro generates the metadata export:

```rust
#![no_std]
#![allow(improper_ctypes_definitions)]

extern crate alloc;

use alloc::vec::Vec;
use foruster_plugin_sdk::prelude::*;

foruster_plugin_sdk::declare_plugin! {
    id: "my-detector",
    name: "My Detector",
    description: "Does something cool",
    descriptions: {
        "es" => "Hace algo genial",
        "fr" => "Fait quelque chose de cool",
    },
    version: "1.0.0",
    author: "Your Name",
    phase: ExecutionPhase::DuringAnalysis,
    category: PluginCategory::Custom,
    result_type: ResultType::ImageDetection,
    extensions: ["jpg", "png"],
    model: None::<&str>,
}

#[no_mangle]
pub extern "C" fn plugin_analyze(req_ptr: i32, req_len: i32)
    -> (i32, i32, i32)
{
    let request: AnalysisRequest = match sdk::read_json(req_ptr, req_len) {
        Ok(r) => r,
        Err(e) => return sdk::return_error(e),
    };
    // … analysis …
    sdk::return_result(&AnalysisResponse { findings: vec![], processing_time_us: 0 })
}
```

See **[PLUGIN_SDK.md](PLUGIN_SDK.md)** for `AnalysisRequest`, `Finding`, and return conventions.

---

## SDK API overview

- **Metadata** — `PluginMetadata` / `declare_plugin!`: identity, categories, supported extensions, optional model id.
- **Analysis** — `plugin_analyze` receives JSON `AnalysisRequest`; return JSON `AnalysisResponse` with `Findings`.
- **Host helpers** — Under `wasm32`, use `foruster_plugin_sdk::host` for file reads, hashes, inference, SQLite, images, logging (see **PLUGIN_SDK.md § 6**).

---

## Host functions

Imports are exposed under Wasm module **`env`**. Do not call raw imports unless you have a reason; use the **`host`** module wrappers. Limits (per-file read caps, per-invocation budgets) are enforced by the desktop host — see **PLUGIN_SDK.md § 6.3**.

---

## Build and test

```bash
cargo build --target wasm32-wasip1 --release
```

Copy the resulting `.wasm` next to the Foruster binary or into `plugins/`. For workspace-wide builds and sample plugins, see **[PLUGIN_BUILD_VERIFY.md](PLUGIN_BUILD_VERIFY.md)**.

---

## Good practices

- Keep **all user-visible strings** for your plugin inside the plugin (metadata maps, bundled strings); the main app does not translate third-party extensions via gettext.
- Use **`host_file_metadata`** before large reads; respect host I/O budgets.
- Prefer **`declare_plugin!`** so metadata and manifest entries stay in sync.

---

## Debugging

- Enable host logging via `host_log` / `plugin_*!` macros.
- Inspect WASM exports/imports with **wasm-objdump** (see **PLUGIN_BUILD_VERIFY.md**).

---

## Support

Use the channel agreed with your distributor or licence terms (this public mirror does not track SDK support tickets).

---

The **`foruster-plugin-sdk`** crate is dual-licensed under **MIT OR Apache-2.0**.
