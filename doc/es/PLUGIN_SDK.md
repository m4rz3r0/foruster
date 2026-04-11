# Referencia del SDK de plugins Foruster

Documenta los **tipos canónicos del SDK** y las convenciones compartidas entre los **plugins WASM**, la aplicación **anfitriona** (binario nativo Foruster), el instalador y la integración continua. El contrato asegura una **ABI** estable entre el código nativo y los módulos WebAssembly, acorde al diseño modular y al aislamiento del entorno de ejecución.

**Idioma:** [English →](../en/PLUGIN_SDK.md)  
**Véase también:** [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) (guía práctica), [INDEX.md](INDEX.md) (índice).

---

## 1. Descripción general

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

La caja del SDK (`foruster-plugin-sdk`) es la **única fuente de verdad**
para todo tipo que cruza un límite:

| Límite | Tipos |
|---|---|
| Plugin ↔ Host (ABI WASM) | `PluginMetadata`, `AnalysisRequest`, `AnalysisResponse`, `Finding`, … |
| Host ↔ Instalador (JSON) | `PluginManifestEntry` (= esquema `plugins-manifest.json`) |
| CI ↔ Instalador (JSON) | `PluginManifestEntry` |

---

## 2. Características de la caja

| Característica | Predeterminado | Descripción |
|---|---|---|
| *(ninguna)* | sí | `no_std` + `alloc`; adecuado para objetivos `wasm32-wasip1`. |
| `std` | no | Habilita la biblioteca estándar; úsese desde código nativo (host/instalador). |

```toml
# In a WASM plugin
[dependencies]
foruster-plugin-sdk = { path = "../plugin-sdk" }

# In a native crate (host, installer, CI tooling)
[dependencies]
foruster-plugin-sdk = { path = "../plugin-sdk", features = ["std"] }
```

---

## 3. Tipos del ABI (`abi.rs`)

### `PluginMetadata`

Lo devuelve la exportación `plugin_metadata` del plugin. El anfitrión valida
`abi_version` y almacena en caché el resto.

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

Para plugins de **IA sobre imágenes**, deje `supported_extensions` vacío y fije `supported_profile_ids` a `["images"]` para que el anfitrión ejecute el plugin solo en rutas que el recorrido de análisis marcó con ese perfil (la tipificación de archivos está centralizada en el recorredor).

En la pantalla de **gestión de extensiones** del escritorio, la aplicación resuelve esos identificadores a los mismos **nombres de perfil localizados** que el selector de perfiles de análisis y el filtro de resultados (mediante `ProfileAPI` y las definiciones de perfil incrustadas o cargadas). Los autores no traducen identificadores de perfil dentro del WASM; el anfitrión mapea `id` → nombre mostrado.

El mapa `descriptions` permite incluir traducciones junto a la
`description` predeterminada en inglés. Las claves son códigos de idioma ISO 639-1
(`"es"`, `"fr"`, …). Tanto el instalador como la aplicación de escritorio
resuelven la configuración regional activa desde este mapa, con retroceso a `description`.

### Traducciones propiedad de la extensión (política)

**Todo texto visible que pertenezca a un plugin debe distribuirse dentro de ese plugin.**
La aplicación Foruster traduce solo su propia interfaz mediante gettext (ficheros `.po` / `.mo`).
**No** mantiene traducciones de extensiones de terceros o propias en esos
catálogos.

- **Metadatos** (`name` / `names`, `description` / `descriptions`, etiquetas de parámetros): mapas
  por código de idioma, incrustados en el WASM o el manifiesto.
- **Salida en tiempo de ejecución** (`Finding::category`, `details`, celdas de tabla, etc.): el anfitrión pasa
  [`AnalysisRequest::locale`] para que el plugin elija la frase correcta entre **cadenas
  incluidas en la extensión** (o devuelva un formato neutro). El anfitrión muestra esos campos
  como texto opaco sin pasarlos por gettext.

Así las extensiones externas siguen siendo utilizables sin parchear los catálogos de traducción de la aplicación principal.

### Enumeraciones

```rust
enum ExecutionPhase { BeforeAnalysis, DuringAnalysis, AfterAnalysis }
enum PluginCategory { AIDetection, DataExtraction, MetadataAnalysis, SystemForensics, Custom }
enum ResultType     { ImageDetection, TableData, ConversationData, TreeData, TimelineData, RawData }
enum Severity       { Info, Low, Medium, High, Critical }
enum ErrorCode      { Success=0, InvalidHandle=-1, InvalidInput=-2, … }
```

### Entrada y salida del análisis

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

El campo `locale` en [`AnalysisRequest`] lo fija el anfitrión según el idioma de la aplicación; los plugins
lo usan para elegir traducciones incrustadas. El anfitrión no traduce esas cadenas.

---

## 4. Tipos del manifiesto (`manifest.rs`)

### `PluginManifestEntry`

Superconjunto de `PluginMetadata` más campos de distribución rellenados por CI o
el instalador:

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

El mapa `descriptions` refleja el de `PluginMetadata`. Al usar
`declare_plugin!`, ambas estructuras se rellenan desde la misma fuente.

### `plugins-manifest.json`

Artefacto de publicación que contiene un vector de `PluginManifestEntry`:

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

El instalador deserializa esto directamente en `Vec<PluginManifestEntry>`.

---

## 5. Macro `declare_plugin!`

Genera las exportaciones WASM `plugin_metadata` y `plugin_manifest_entry`
a partir de una sola declaración:

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

El bloque `descriptions` es **opcional**. Omítalo por completo si solo
aporta descripción en inglés — la macro usa un mapa vacío por defecto.

Sustituye la función manual `plugin_metadata` de plugins antiguos
y garantiza que los metadatos y la entrada de manifiesto permanezcan
alineados.

---

## 6. Funciones del anfitrión (solo WASM)

El *runtime* de Foruster registra las importaciones Wasm bajo el módulo **`env`**. Las firmas y el diseño de `ret_area` coinciden con el cargador nativo. La inferencia (`host_run_inference`) se ejecuta en el **motor de inferencia** de la aplicación anfitriona (ONNX Runtime), **en el propio equipo**.

### 6.1 ABI de importación Wasm (canónica)

| Importación | Firma Wasm | Retorno / notas |
|--------|----------------|----------------|
| `host_read_file` | `(ret_area: i32, handle: i64) → ()` | `ret_area`: 12 bytes (`ptr`, `len`, `error_code`, *little-endian*). Si la llamada tiene éxito, el invitado lee desde `ptr`/`len`. |
| `host_log` | `(level: i32, msg_ptr: i32, msg_len: i32) → ()` | `level`: de *trace* (0) a *error* (4); véase `LogLevel`. Mensaje UTF-8 en memoria del invitado. |
| `host_file_metadata` | `(ret_area: i32, handle: i64) → ()` | Igual estructura `ret_area` que `read_file`. Devuelve JSON con `FileMetadata` (`size`, `extension`, `mime_type` / `blake3_hash` opcionales). |
| `host_compute_hash` | `(ret_area: i32, handle: i64) → ()` | Misma estructura. Respuesta: hash **BLAKE3** en hexadecimal (ASCII), calculado en el anfitrión. |
| `host_run_inference` | `(ret_area, model_ptr, model_len, tensor_ptr, tensor_len, shape_ptr, shape_len) → ()` | Misma estructura `ret_area`. Salida: tensor `f32`; la forma la determina el anfitrión o el modelo. |
| `host_current_time` | `() → i64` | Tiempo Unix (segundos). |
| `host_query_sqlite` | `(ret_area, handle, query_ptr, query_len) → ()` | Misma estructura. JSON `SqliteResult` (`columns` + `rows`). |
| `host_decode_image` | `(ret_area, handle, target_w, target_h) → ()` | `ret_area`: **20 bytes** (`ptr`, `len`, `orig_w`, `orig_h`, `error_code`; `write_ret5` en el anfitrión). Tensor CHW `f32`, normalizado entre 0 y 1. |

Los plugins compilados para `wasm32` deben usar la API Rust **`foruster-plugin-sdk::host`** que sigue en lugar de invocar estas importaciones manualmente.

### 6.2 Envoltorios Rust (`foruster_plugin_sdk::host`)

Disponibles al compilar para **`wasm32`** (véase el módulo `host` del SDK):

| Función | Finalidad |
|----------|---------|
| `read_file(handle)` | Contenido completo del archivo asociado al `file_handle` del análisis. |
| `file_metadata(handle)` | `FileMetadata` a partir del JSON del anfitrión. |
| `compute_hash(handle)` | Hash BLAKE3 del archivo en hexadecimal. |
| `run_inference(model_id, tensor, shape)` | Inferencia ONNX en el anfitrión (ONNX Runtime). |
| `decode_image(handle, target_w, target_h)` | Decodifica y redimensiona; devuelve tensor CHW `f32` y dimensiones originales. |
| `query_sqlite(handle, query)` | Consulta SQL de solo lectura sobre SQLite (p. ej. bases al estilo NSRL). |
| `current_time()` | Tiempo Unix (`i64`). |
| `log(level, msg)` | Registro estructurado en el anfitrión. |

Macros de conveniencia: `plugin_info!`, `plugin_warn!`, `plugin_error!` (y `plugin_log!`).

### 6.3 Límites en la aplicación anfitriona (Foruster nativo)

La aplicación de escritorio impone **límites adicionales** (no forman parte del contrato ABI WASM, pero son política estable en esta publicación):

| Política | Valor | Notas |
|--------|-------|--------|
| Límite por archivo (`host_read_file`, `host_compute_hash`, `host_decode_image`) | 256 MiB | Se rechaza con `InvalidInput` si el tamaño en disco lo supera. |
| Límite acumulado de lectura por llamada a `plugin_analyze` | 1 GiB | Suma de todas las lecturas del anfitrión en esa invocación; por encima, `OutOfMemory`. |
| Restablecimiento del contador | Cada `plugin_analyze` | Antes de invocar al invitado se llama a `reset_host_io_budget()`. |

Los autores de plugins deben usar `host_file_metadata` para comprobar el tamaño antes de asumir lecturas completas. Las bases de datos de hashes y los archivos muy grandes pueden requerir preprocesado en el anfitrión o **conjuntos de prueba** más reducidos.

---

## 7. Gestión de memoria

Los plugins usan un asignador por regiones (`memory.rs`). El anfitrión invoca
`plugin_reset()` antes de cada llamada a `plugin_analyze` para liberar todas las
asignaciones temporales y evitar fugas entre llamadas.

Funciones exportadas usadas por el anfitrión:

| Exportación | Finalidad |
|---|---|
| `plugin_alloc(size) → *mut u8` | Asignar memoria del invitado |
| `plugin_free(ptr, size)` | Liberar una asignación |
| `plugin_reset()` | Liberar de golpe todas las asignaciones de la región |

Los autores de plugins **no** deben llamarlas directamente; los auxiliares del SDK
(`return_json`, `return_result`, etc.) gestionan la asignación de forma transparente.

---

## 8. Versiones y compatibilidad

- **Versión del ABI** (`WASM_ABI_VERSION`): actualmente `(1, 0)`. El anfitrión
  rechaza plugins cuya versión mayor difiera.
- **Versión del SDK** (`foruster-plugin-sdk`): sigue semver de forma independiente
  de la aplicación principal (véase `plugin-sdk/CHANGELOG.md`).
- **Versión del plugin**: se declara por plugin en `PluginMetadata.version`.
- **Cadena de herramientas (anfitrión frente a SDK):** el espacio de trabajo principal de Foruster apunta a un **Rust estable reciente** (véase el `Cargo.toml` raíz). La caja `plugin-sdk` puede usar un **`rust-version` y `edition` más antiguos** para que plugins de terceros compilen con un abanico más amplio de compiladores; los invitados siguen compilándose para **`wasm32-wasip1`**. Ante la duda, use la misma versión mayor de Rust que la aplicación anfitriona para compilaciones reproducibles.

Las versiones de paquetes de extensiones se publican junto con las publicaciones de plugins.

---

## 9. Flujo de publicación

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

El instalador (modo en línea) obtiene `plugins-manifest.json`, lo deserializa
en `Vec<PluginManifestEntry>` y muestra la lista para su selección.

---

## 10. Licencia

El SDK en sí se publica con doble licencia **MIT OR Apache-2.0**.

Los plugins pueden usar cualquier licencia compatible con su caso de uso.
