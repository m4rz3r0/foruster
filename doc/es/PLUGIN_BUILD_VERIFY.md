# Compilar y verificar plugins WASM

Este documento complementa [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) y [PLUGIN_SDK.md](PLUGIN_SDK.md). Explica cómo compilar el espacio de trabajo, generar **plugins WASM** de ejemplo (módulos que amplían la aplicación sin recompilar el núcleo) y ejecutar las pruebas asociadas, sin rutas dependientes de la máquina. La **extensibilidad por plugins** forma parte de la arquitectura modular descrita en el *Trabajo Fin de Máster* (2026).

**Idioma:** [English →](../en/PLUGIN_BUILD_VERIFY.md)

## Requisitos previos

- Cadena de herramientas Rust acorde al espacio de trabajo (`rust-version` en el `Cargo.toml` raíz).
- Objetivo WebAssembly:

```bash
rustup target add wasm32-wasip1
```

## 1. Compilar el espacio de trabajo principal

Desde la raíz del repositorio:

```bash
cargo build --workspace
```

Análisis estático opcional:

```bash
cargo clippy --workspace -- -W clippy::all
```

Si las versiones de dependencias divergen (p. ej. Wasmtime, `ort`), alinee los pines del cargador nativo y del espacio de trabajo con la publicación del SDK admitida.

## 2. Compilar plugins de ejemplo

Con Makefile (shell POSIX):

```bash
make plugins
```

O ejecute `./scripts/build-plugins.sh` (equivalente a `make plugins`) o `./scripts/build-plugins.fish` en Fish. Los `.wasm` generados se copian bajo `plugins/`.

Para compilar un ejemplo manualmente:

```bash
cd examples/simple-detector
cargo build --target wasm32-wasip1 --release
```

Copie el `.wasm` resultante junto al binario de Foruster o en `plugins/` como se describe en [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md).

## 3. Inspeccionar el módulo

Con [WABT](https://github.com/WebAssembly/wabt) (`wasm-objdump`):

```bash
wasm-objdump -x plugins/simple_detector.wasm | grep -A 20 Export
wasm-objdump -x plugins/simple_detector.wasm | grep -A 10 Import
```

Las exportaciones deben incluir los símbolos que exige el anfitrión (véase [PLUGIN_SDK.md](PLUGIN_SDK.md)).

## 4. Pruebas

Pruebas del espacio de trabajo:

```bash
cargo test --workspace
```

Las pruebas de integración de plugins van con el **repositorio de desarrollo completo** (no con este espejo). Si tiene acceso, ejecute las pruebas descritas allí.

## 5. Resolución de problemas

**No se encuentra la caja `foruster_plugin_sdk`**

Compile el SDK desde `plugin-sdk/`:

```bash
cargo check --manifest-path plugin-sdk/Cargo.toml
```

O use `bash scripts/check-plugin-sdk.sh`.

**Desajuste de versión de Wasmtime**

Alinee las dependencias relacionadas con Wasmtime con el conjunto usado en la compilación publicada del anfitrión.

**El plugin no se detecta**

Asegúrese de que los `.wasm` estén en `plugins/` respecto al directorio de trabajo o el diseño descrito en [FORENSIC_POLICY.md](FORENSIC_POLICY.md).

**Problemas con ONNX / `ort`**

Véase [STATIC_BUILDS.md](STATIC_BUILDS.md) y el `Cargo.toml` raíz para la configuración de dependencias de inferencia.
