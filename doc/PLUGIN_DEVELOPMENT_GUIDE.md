# Foruster Plugin Development Guide

Guía completa para desarrollar plugins WASM para Foruster.

## Tabla de Contenidos

1. [Setup del Entorno](#setup-del-entorno)
2. [Tu Primer Plugin](#tu-primer-plugin)
3. [API del SDK](#api-del-sdk)
4. [Host Functions](#host-functions)
5. [Compilación y Testing](#compilación-y-testing)
6. [Best Practices](#best-practices)
7. [Debugging](#debugging)

---

## Setup del Entorno

### Requisitos

- Rust 1.87+ con target `wasm32-wasip1`
- Foruster SDK (`foruster-plugin-sdk`)

### Instalación

```bash
# Instalar target WASM
rustup target add wasm32-wasip1

# Crear nuevo plugin
cargo new --lib my-detector
cd my-detector
```

### Cargo.toml

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
opt-level = "z"     # Size optimization
lto = true
strip = true
```

---

## Tu Primer Plugin

### Estructura Mínima

Todo plugin debe export `plugin_metadata` (identity) and
`plugin_analyze` (logic). The `declare_plugin!` macro generates the
metadata export for you:

```rust
#![no_std]
#![allow(improper_ctypes_definitions)]

extern crate alloc;

use alloc::vec::Vec;
use foruster_plugin_sdk::prelude::*;

// 1. Plugin declaration (generates plugin_metadata + plugin_manifest_entry)
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

// 2. Analysis function
#[no_mangle]
pub extern "C" fn plugin_analyze(req_ptr: i32, req_len: i32)
    -> (i32, i32, i32)
{
    let request: AnalysisRequest = match sdk::read_json(req_ptr, req_len) {
        Ok(r) => r,
        Err(e) => return sdk::return_error(e),
    };

    // Your analysis logic here
    let findings = Vec::new();

    let response = AnalysisResponse {
        findings,
        processing_time_us: 100,
    };

    sdk::return_result(&response)
}
```

> See [PLUGIN_SDK.md](PLUGIN_SDK.md) for the full type reference and
> manifest format.

---

## API del SDK

### Tipos Principales

#### PluginMetadata

```rust
pub struct PluginMetadata {
    pub abi_version: (u32, u32),        // Siempre (1, 0)
    pub id: String,                      // Identificador único (slug)
    pub name: String,                    // Nombre legible
    pub description: String,             // Descripción breve (inglés)
    pub descriptions: BTreeMap<String, String>, // Traducciones por código ISO 639-1
    pub version: String,                 // Semver (e.g., "1.0.0")
    pub author: String,                  // Tu nombre/organización
    pub execution_phase: ExecutionPhase, // Cuándo ejecutar
    pub category: PluginCategory,        // Categoría del plugin
    pub result_type: ResultType,         // Tipo de UI para resultados
    pub supported_extensions: Vec<String>, // Extensiones soportadas
    pub requires_model: Option<String>,  // Si necesita modelo ONNX
}
```

El campo `descriptions` permite incluir traducciones de la descripción.
Tanto el instalador como la aplicación de escritorio seleccionan
automáticamente el idioma activo del mapa, con fallback a `description`.

#### ExecutionPhase

```rust
pub enum ExecutionPhase {
    BeforeAnalysis,  // Antes del scan (setup, config)
    DuringAnalysis,  // Durante scan (standard)
    AfterAnalysis,   // Después del scan (AI pesado)
}
```

#### PluginCategory

```rust
pub enum PluginCategory {
    AIDetection,        // Detección con IA
    DataExtraction,     // Extracción de datos (cookies, logs)
    MetadataAnalysis,   // Análisis de metadatos
    SystemForensics,    // Análisis del sistema
    Custom,             // Personalizado
}
```

#### Finding

```rust
pub struct Finding {
    pub category: String,           // e.g., "weapon", "nsfw"
    pub confidence: f32,            // 0.0 - 1.0
    pub severity: Severity,         // Info/Low/Medium/High/Critical
    pub details: Option<String>,    // Información adicional
    pub bbox: Option<BBox>,         // Para detecciones con bounding box
}
```

### Funciones Helper

```rust
// Serializar y retornar JSON
sdk::return_json(&metadata) -> (i32, i32)

// Parsear JSON del host
sdk::read_json::<T>(ptr, len) -> Result<T, ErrorCode>

// Retornar error
sdk::return_error(ErrorCode::InvalidInput) -> (i32, i32, i32)

// Retornar resultado exitoso
sdk::return_result(&response) -> (i32, i32, i32)
```

---

## Host Functions

Funciones que el plugin puede llamar al host:

### host::read_file()

```rust
use foruster_plugin_sdk::host;

let content = match host::read_file(request.file_handle) {
    Ok(bytes) => bytes,
    Err(e) => return sdk::return_error(e),
};
```

### host::log()

```rust
use foruster_plugin_sdk::{host, plugin_info, plugin_warn, plugin_error};

// Directo
host::log(LogLevel::Info, "Processing file");

// Con macros (recomendado)
plugin_info!("Analyzing {} bytes", content.len());
plugin_warn!("Unusual file size");
plugin_error!("Failed to decode image");
```

### host::run_inference() *(Próximamente)*

```rust
// Para plugins con IA
let output = host::run_inference(
    "nsfw-v1",          // Model ID
    input_tensor,       // Vec<f32>
    vec![1, 3, 224, 224], // Shape
)?;
```

---

## Compilación y Testing

### Compilar

```bash
# Debug build
cargo build --target wasm32-wasip1

# Release build (optimizado)
cargo build --target wasm32-wasip1 --release

# El .wasm estará en:
# target/wasm32-wasip1/release/my_detector.wasm
```

### Instalar en Foruster

```bash
cp target/wasm32-wasip1/release/my_detector.wasm \
   /path/to/foruster/plugins/
```

### Testing Local

Crear un test runner (con feature `std`):

```rust
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    
    #[test]
    fn test_metadata() {
        let (ptr, len) = plugin_metadata();
        assert!(len > 0);
        // Parsear y validar JSON
    }
}
```

---

## Best Practices

### 1. Manejo de Errores Robusto

```rust
pub extern "C" fn plugin_analyze(req_ptr: i32, req_len: i32) 
    -> (i32, i32, i32) {
    
    // SIEMPRE validar inputs
    let request: AnalysisRequest = match sdk::read_json(req_ptr, req_len) {
        Ok(r) => r,
        Err(e) => {
            plugin_error!("Invalid request JSON");
            return sdk::return_error(e);
        }
    };
    
    // SIEMPRE manejar errores de host functions
    let content = match host::read_file(request.file_handle) {
        Ok(bytes) => bytes,
        Err(e) => {
            plugin_error!("Failed to read file: {:?}", e);
            return sdk::return_error(e);
        }
    };
    
    // NUNCA panic - retornar error
    // ❌ let data = content[0]; // puede panic
    // ✅ let data = content.get(0).copied().unwrap_or(0);
    
    // ...
}
```

### 2. Logging Informativo

```rust
plugin_info!("Starting analysis of {} bytes", content.len());
plugin_info!("Detected format: {:?}", format);
plugin_info!("Found {} potential matches", count);
```

### 3. Performance

```rust
// ✅ Reutilizar allocaciones
let mut findings = Vec::with_capacity(10);

// ✅ Evitar copias innecesarias
let text = core::str::from_utf8(&content)?;

// ❌ Evitar conversiones pesadas en loop
for line in text.lines() {
    let lower = line.to_lowercase(); // Copia!
}
```

### 4. Validación de Inputs

```rust
// Validar tamaño
if content.len() > 100_000_000 { // 100 MB
    plugin_warn!("File too large, skipping");
    return sdk::return_result(&AnalysisResponse {
        findings: Vec::new(),
        processing_time_us: 0,
    });
}

// Validar formato
if !is_valid_image(&content) {
    return sdk::return_error(ErrorCode::UnsupportedFormat);
}
```

---

## Debugging

### Logs

Los logs del plugin aparecen en la consola de Foruster con prefijo `[Plugin]`:

```
[2024-04-07 19:30:15] INFO [Plugin] Analyzing file handle 42
[2024-04-07 19:30:15] INFO [Plugin] Found keyword: secreto
[2024-04-07 19:30:15] INFO [Plugin] Found 3 sensitive keywords
```

### Errores Comunes

#### 1. "Plugin must export 'memory'"

**Problema**: El plugin no exporta memoria lineal.

**Solución**: Asegúrate de compilar como `cdylib`:

```toml
[lib]
crate-type = ["cdylib"]
```

#### 2. "ABI version mismatch"

**Problema**: Versión del SDK no coincide con Foruster.

**Solución**: Actualizar `foruster-plugin-sdk` y recompilar.

#### 3. "Invalid response JSON"

**Problema**: La respuesta no se puede parsear.

**Solución**: Usar `sdk::return_result()` en lugar de serializar manualmente.

### Inspeccionar WASM

```bash
# Ver exports del módulo
wasm-objdump -x my_detector.wasm | grep export

# Ver imports
wasm-objdump -x my_detector.wasm | grep import

# Tamaño del binario
ls -lh target/wasm32-wasip1/release/my_detector.wasm
```

---

## Ejemplos Completos

Ver `examples/` para plugins funcionales:

- `nsfw-detector-wasm/` - AI-based NSFW/CSAM content detector
- `weapon-detector-wasm/` - YOLOv8-based weapon detector (pistols, knives)
- `cookie-extractor-wasm/` - Browser cookie database extraction

---

## Soporte

- **Documentación**: `doc/WASM_PLUGIN_ARCHITECTURE.md`
- **Support**: use the channel agreed with your distributor or license terms (this public mirror does not track SDK support tickets).
- **Discord**: *(próximamente)*

---

## Licencia del SDK

El SDK (`foruster-plugin-sdk`) está licenciado bajo MIT OR Apache-2.0.

Los plugins pueden usar cualquier licencia compatible con su caso de uso (Open Source o Propietario).
