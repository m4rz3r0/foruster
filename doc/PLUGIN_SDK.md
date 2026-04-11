# Foruster Plugin SDK Reference

This document describes the **canonical SDK types** and conventions that
all Foruster WASM plugins, the host runtime, and the installer share.

**See also:** [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) (tutorial), [INDEX.md](INDEX.md) (documentation index).

---

## 1. Overview

```
┌──────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  WASM Plugin │     │  Foruster Host   │     │  Installer / CI  │
│  (no_std)    │     │  (native, std)   │     │  (native, std)   │
└──────┬───────┘     └────────┬─────────┘     └────────┬─────────┘
       │                      │                        │
       └──────────┬───────────┴────────────────────────┘
                  │
        ┌─────────▼──────────┐
        │ foruster-plugin-sdk │
        │                    │
        │  abi.rs            │  ABI types (PluginMetadata, Finding, …)
        │  manifest.rs       │  PluginManifestEntry (release metadata)
        │  sdk.rs            │  Helpers (return_json, manifest_from_metadata)
        │  host.rs  [wasm32] │  Host import wrappers (read_file, inference, SQLite, …)
        │  memory.rs [wasm32]│  Arena allocator for guest↔host exchange
        └────────────────────┘
```

The SDK crate (`foruster-plugin-sdk`) is the **single source of truth**
for every type that crosses a boundary:

| Boundary | Types used |
|---|---|
| Plugin ↔ Host (WASM ABI) | `PluginMetadata`, `AnalysisRequest`, `AnalysisResponse`, `Finding`, … |
| Host ↔ Installer (JSON) | `PluginManifestEntry` (= `plugins-manifest.json` schema) |
| CI ↔ Installer (JSON) | `PluginManifestEntry` |

---

## 2. Crate features

| Feature | Default | Description |
|---|---|---|
| *(none)* | yes | `no_std` + `alloc`; suitable for `wasm32-wasip1` targets. |
| `std` | no | Enables standard library; use from native (host/installer) code. |

```toml
# In a WASM plugin
[dependencies]
foruster-plugin-sdk = { path = "../plugin-sdk" }

# In a native crate (host, installer, CI tooling)
[dependencies]
foruster-plugin-sdk = { path = "../plugin-sdk", features = ["std"] }
```

---

## 3. ABI types (`abi.rs`)

### `PluginMetadata`

Returned by the plugin's `plugin_metadata` export. The host validates
`abi_version` and caches the rest.

```rust
pub struct PluginMetadata {
    pub abi_version: (u32, u32),         // Must match WASM_ABI_VERSION major
    pub id: String,                       // Unique slug (e.g. "nsfw-detector")
    pub name: String,                     // Human-readable name
    pub description: String,              // English default
    pub descriptions: BTreeMap<String, String>, // Localised overrides (optional)
    pub version: String,                  // Semver
    pub author: String,
    pub execution_phase: ExecutionPhase,
    pub category: PluginCategory,
    pub result_type: ResultType,
    pub supported_extensions: Vec<String>, // Lowercase, no dot; empty ⇒ no suffix filter
    pub supported_profile_ids: Vec<String>, // e.g. ["images"] for image AI; host matches walk IDs
    pub requires_model: Option<String>,   // ONNX model ID (e.g. "nsfw-v1")
}
```

For **image AI** plugins, leave `supported_extensions` empty and set `supported_profile_ids` to `["images"]` so the host only runs the plugin on paths the analysis walk tagged with that profile (file typing is centralized in the walker).

On the **desktop extension management** screen, the app resolves those IDs to the same **localized profile names** shown in the analysis profile picker and the results filter (via `ProfileAPI` and the embedded / loaded profile definitions). Authors do not translate profile IDs inside the WASM; the host maps `id` → display name.

The `descriptions` map lets plugin authors ship translations alongside
the default English `description`. Keys are ISO 639-1 language codes
(`"es"`, `"fr"`, …). Both the installer and the desktop application
resolve the active locale from this map, falling back to `description`.

### Extension-owned translations (policy)

**All user-visible text that belongs to a plugin must ship inside that plugin.**
The Foruster application translates only its own UI via gettext (`.po` / `.mo` files).
It does **not** maintain translations for third-party or first-party extensions in those
catalogues.

- **Metadata** (`name` / `names`, `description` / `descriptions`, parameter labels): maps
  keyed by language code, embedded in the WASM or manifest.
- **Runtime output** (`Finding::category`, `details`, table cells, etc.): the host passes
  [`AnalysisRequest::locale`] so the plugin can select the correct phrase from **strings
  bundled in the extension** (or return a neutral format). The host displays those fields
  as opaque text without running them through gettext.

This keeps external extensions usable without patching the main app’s translation files.

### Enums

```rust
enum ExecutionPhase { BeforeAnalysis, DuringAnalysis, AfterAnalysis }
enum PluginCategory { AIDetection, DataExtraction, MetadataAnalysis, SystemForensics, Custom }
enum ResultType     { ImageDetection, TableData, ConversationData, TreeData, TimelineData, RawData }
enum Severity       { Info, Low, Medium, High, Critical }
enum ErrorCode      { Success=0, InvalidHandle=-1, InvalidInput=-2, … }
```

### Analysis I/O

```rust
pub struct AnalysisRequest {
    pub file_handle: u64,
    pub file_path: String,
    pub file_size: u64,
    pub extension: Option<String>,
    pub mime_type: Option<String>,
    pub parameters: BTreeMap<String, String>,
    pub locale: String,
}

pub struct AnalysisResponse {
    pub findings: Vec<Finding>,
    pub processing_time_us: u64,
}

pub struct Finding {
    pub category: String,       // Opaque: localized by the plugin, not the app
    pub confidence: f32,        // 0.0 – 1.0
    pub severity: Severity,
    pub details: Option<String>, // Opaque: localized by the plugin, not the app
    pub bbox: Option<BBox>,
}
```

`locale` on [`AnalysisRequest`] is set by the host from the app language setting; plugins
use it to choose embedded translations. The host does not translate these strings.

---

## 4. Manifest types (`manifest.rs`)

### `PluginManifestEntry`

Superset of `PluginMetadata` plus distribution fields filled by CI or
the installer:

```rust
pub struct PluginManifestEntry {
    // ── Identity (same as PluginMetadata minus abi_version) ──
    pub id: String,
    pub name: String,
    pub description: String,              // English default
    pub descriptions: BTreeMap<String, String>, // Localised overrides (optional)
    pub version: String,
    pub author: String,

    // ── Capabilities ──
    pub category: PluginCategory,
    pub result_type: ResultType,
    pub execution_phase: ExecutionPhase,
    pub supported_extensions: Vec<String>,
    pub requires_model: Option<String>,

    // ── Distribution (optional, filled by CI / installer) ──
    pub file: Option<String>,          // e.g. "nsfw_detector_wasm.wasm"
    pub size: Option<u64>,             // bytes
    pub sha256: Option<String>,        // hex-encoded hash
    pub download_url: Option<String>,  // browser_download_url from GitHub
}
```

The `descriptions` map mirrors the one in `PluginMetadata`. When using
`declare_plugin!`, both structs are populated from the same source.

### `plugins-manifest.json`

A release artifact containing an array of `PluginManifestEntry`:

```json
[
  {
    "id": "nsfw-detector",
    "name": "NSFW Detector",
    "description": "Detects NSFW/CSAM content in images using AI",
    "descriptions": {
      "es": "Detecta contenido NSFW/CSAM en imágenes usando IA",
      "fr": "Détecte le contenu NSFW/CSAM dans les images par IA"
    },
    "version": "1.0.0",
    "author": "Foruster Team",
    "category": "AIDetection",
    "result_type": "ImageDetection",
    "execution_phase": "DuringAnalysis",
    "supported_extensions": [],
    "supported_profile_ids": ["images"],
    "requires_model": "nsfw-v1",
    "file": "nsfw_detector_wasm.wasm",
    "size": 48320,
    "sha256": "a1b2c3…"
  }
]
```

The installer deserialises this directly into `Vec<PluginManifestEntry>`.

---

## 5. `declare_plugin!` macro

Generates both `plugin_metadata` and `plugin_manifest_entry` WASM
exports from a single declaration:

```rust
#![no_std]
extern crate alloc;

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
    result_type: ResultType::RawData,
    extensions: ["txt", "log"],
    model: None::<&str>,
}

#[no_mangle]
pub extern "C" fn plugin_analyze(req_ptr: i32, req_len: i32)
    -> (i32, i32, i32)
{
    // … analysis logic …
}
```

The `descriptions` block is **optional**. Omit it entirely if you only
provide an English description — the macro defaults to an empty map.

This replaces the manual `plugin_metadata` function seen in older
plugins and ensures the metadata and manifest entry are always
consistent.

---

## 6. Host functions (WASM only)

Imports are provided by the Foruster runtime under the Wasm module name **`env`**.  
The canonical signatures and `ret_area` layouts are registered by the
Foruster host when loading guests.

### 6.1 Wasm import ABI (canonical)

| Import | Wasm signature | Return / notes |
|--------|----------------|----------------|
| `host_read_file` | `(ret_area: i32, handle: i64) → ()` | `ret_area` is 12 bytes: `ptr: i32`, `len: i32`, `error_code: i32` (little-endian). Success: guest reads bytes at `ptr`/`len`. |
| `host_log` | `(level: i32, msg_ptr: i32, msg_len: i32) → ()` | `level`: trace=0 … error=4 (see `LogLevel`). Message UTF-8 in guest memory. |
| `host_file_metadata` | `(ret_area: i32, handle: i64) → ()` | Same `ret_area` triple as `read_file`. Payload is JSON for `FileMetadata` (`size`, `extension`, optional `mime_type` / `blake3_hash`). |
| `host_compute_hash` | `(ret_area: i32, handle: i64) → ()` | Same triple. Payload is **hex-encoded BLAKE3** of the file (ASCII), computed on the host. |
| `host_run_inference` | `(ret_area, model_ptr, model_len, tensor_ptr, tensor_len, shape_ptr, shape_len) → ()` | Same `ret_area` triple. Output buffer is `f32` tensor bytes; shape is implied by host/model. |
| `host_current_time` | `() → i64` | Unix time in seconds. |
| `host_query_sqlite` | `(ret_area, handle, query_ptr, query_len) → ()` | Same triple. JSON result matching `SqliteResult` (`columns` + `rows`). |
| `host_decode_image` | `(ret_area, handle, target_w, target_h) → ()` | `ret_area` is **20 bytes**: `ptr`, `len`, `orig_w`, `orig_h`, `error_code` (`write_ret5` in the host). Tensor: CHW `f32`, normalized 0..1. |

Plugins compiled for `wasm32` should use the **`foruster-plugin-sdk::host`** Rust API below instead of calling these imports by hand.

### 6.2 Rust wrappers (`foruster_plugin_sdk::host`)

Available when building for **`wasm32`** (see the SDK `host` module):

| Function | Purpose |
|----------|---------|
| `read_file(handle)` | Full file bytes for the analysis `file_handle`. |
| `file_metadata(handle)` | `FileMetadata` parsed from host JSON. |
| `compute_hash(handle)` | BLAKE3 hex string for the file. |
| `run_inference(model_id, tensor, shape)` | ONNX inference on the host. |
| `decode_image(handle, target_w, target_h)` | Decode + resize → CHW `f32` tensor and original dimensions. |
| `query_sqlite(handle, query)` | Run a read-only SQL query on a SQLite file (NSRL-style DBs). |
| `current_time()` | Unix time (`i64`). |
| `log(level, msg)` | Structured log line to the host. |

Convenience macros: `plugin_info!`, `plugin_warn!`, `plugin_error!` (and `plugin_log!`).

### 6.3 Host enforcement (native Foruster)

The desktop application applies **additional limits** in the host (not part of the WASM ABI contract, but stable policy for this release):

| Policy | Value | Notes |
|--------|-------|--------|
| Max bytes per file for `host_read_file`, `host_compute_hash`, `host_decode_image` | 256 MiB | Rejects with `InvalidInput` if on-disk size exceeds this. |
| Max total bytes read from disk per `plugin_analyze` call | 1 GiB | Cumulative across host imports; further reads return `OutOfMemory`. |
| Budget reset | Each `plugin_analyze` | `reset_host_io_budget()` runs before guest `plugin_analyze`. |

Plugins should use `host_file_metadata` to check size before relying on full reads. Hash databases and large media may require host-side preprocessing or smaller test fixtures.

---

## 7. Memory management

Plugins use an arena allocator (`memory.rs`). The host calls
`plugin_reset()` before each `plugin_analyze` invocation to free all
temporary allocations, preventing leaks across calls.

Exported functions used by the host:

| Export | Purpose |
|---|---|
| `plugin_alloc(size) → *mut u8` | Allocate guest memory |
| `plugin_free(ptr, size)` | Free a single allocation |
| `plugin_reset()` | Bulk-free all arena allocations |

Plugin authors **do not** need to call these directly; the SDK helpers
(`return_json`, `return_result`, etc.) handle allocation transparently.

---

## 8. Versioning and compatibility

- **ABI version** (`WASM_ABI_VERSION`): currently `(1, 0)`. The host
  rejects plugins whose major version differs.
- **SDK version** (`foruster-plugin-sdk`): follows semver independently
  of the main application version (see `plugin-sdk/CHANGELOG.md`).
- **Plugin version**: declared per-plugin in `PluginMetadata.version`.
- **Host vs SDK toolchain:** The main Foruster workspace targets a **recent stable Rust** (see the root `Cargo.toml`). The `plugin-sdk` crate may use an **older `rust-version` and `edition`** so that third-party plugins can build with a wider range of compilers; guests still compile for **`wasm32-wasip1`**. When in doubt, use the same major Rust release as the host for reproducible builds.

Extension bundle versions are published together with plugin releases.

---

## 9. Release workflow

```
Tag plugins/vX.Y.Z  ──►  CI builds WASM plugins
                              │
                              ├─ generates plugins-manifest.json
                              │    (array of PluginManifestEntry)
                              │
                              ├─ computes SHA256SUMS
                              │
                              └─ publishes manifests and assets to the release channel
```

The installer (online mode) fetches `plugins-manifest.json`, deserialises
it into `Vec<PluginManifestEntry>`, and presents the list for selection.

---

## 10. License

The SDK itself is dual-licensed under **MIT OR Apache-2.0**.

Plugins may use any license compatible with their use case.
