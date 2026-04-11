# Compilaciones estáticas y binarios portátiles

Objetivo: obtener **binarios nativos** portátiles que, en la medida de lo posible, dependan solo del ejecutable y del núcleo del sistema operativo, sin arrastrar copias adicionales de `libc`, el runtime MSVC u otras DLL, salvo las que se documenten explícitamente. El **motor de inferencia** ONNX Runtime suele distribuirse junto al ejecutable: la inferencia se ejecuta **en el propio equipo**, no en servidores remotos.

**Idioma:** [English →](../en/STATIC_BUILDS.md)

## Linux: musl y enlace totalmente estático

- **Objetivo:** `x86_64-unknown-linux-musl` (o `aarch64-unknown-linux-musl` en anfitriones Apple Silicon con cadenas cruzadas).
- **Rust:** `rustup target add x86_64-unknown-linux-musl` e instalar enlazador musl (p. ej. `musl-gcc` / paquetes de distribución, o `cross` de `cross-rs`).
- **Banderas habituales:** `RUSTFLAGS='-C target-feature=+crt-static'` produce un ejecutable estático cuando todas las dependencias nativas lo permiten.

### Limitaciones (compilación desde fuentes)

| Dependencia | Notas |
|-------------|--------|
| **Slint** | Usa backends de plataforma (p. ej. X11/Wayland, fontconfig en muchos escritorios Linux). Una compilación musl puede seguir necesitando **bibliotecas del sistema** en tiempo de ejecución salvo que use un backend empaquetado — verifique con su versión y *feature flags* de Slint. |
| **`ort` (ONNX Runtime)** | La caja `ort` suele cargar bibliotecas **dinámicas** de ONNX Runtime o incluir `.so` precompilados. El enlace **totalmente estático** de ONNX + Slint + pilas GPU suele ser **poco práctico**; trate «binario Rust mayormente estático + `libonnxruntime.so` documentada» como un suelo realista salvo compilación avanzada desde fuentes. |

### Compilaciones sin red / sin descargas (`ort`)

En una copia habitual del repositorio, `ort` suele activarse con **`download-binaries`** para que `cargo build` pueda obtener ONNX Runtime cuando haga falta. Es cómodo en desarrollo pero puede ser indeseable en entornos **sin red** o en **kits forenses reproducibles**.

Opciones:

1. **Proveer ONNX Runtime** — Instale el paquete oficial para su plataforma (o copie `libonnxruntime.so` / `.dll` / `.dylib` desde una compilación de confianza) y apunte el cargador. Ajuste los **feature flags** de `ort` en el `Cargo.toml` raíz para desactivar `download-binaries` y use el conjunto que documente su versión de `ort` para bibliotecas **del sistema** o **manuales** (véase la [documentación de `ort`](https://docs.rs/ort)).
2. **Variables de entorno** — Muchas configuraciones de `ort` / ONNX Runtime respetan variables como `ORT_DYLIB_PATH` (el nombre puede variar) para cargar una biblioteca concreta; fíjela en compilación o en tiempo de ejecución tras colocar la biblioteca junto al ejecutable o en la carpeta del kit.
3. **Documente el artefacto** — En cada publicación, registre la versión de ONNX Runtime y la ruta junto al binario (véase [FORENSIC_POLICY.md](FORENSIC_POLICY.md) para el diseño portátil).

Tras cambiar los *features* de `ort`, ejecute `cargo build --release` en una máquina limpia **sin** red para confirmar que no se descarga nada en tiempo de compilación. Incluso con musl, la pila de interfaz puede exigir `fontconfig`, `freetype` o similares en tiempo de ejecución salvo que esté construida para incrustarlas o evitarlas.

**Recomendación:** Documente la salida real de `ldd` (o equivalente) por artefacto de publicación. Prefiera «Rust **mayormente estático** + lista documentada de `.so`» a prometer un único archivo sin dependencias salvo que lo verifique.

## Windows: runtime MSVC

- Las compilaciones **MSVC** por defecto enlazan con el **Universal C Runtime** y pueden requerir **VC++ Redistributable** en la máquina destino salvo enlace con **`/MT`** (CRT estático) para la cadena Rust y todas las dependencias C++.
- **Slint** y dependencias **Win32** deben compilarse con configuraciones CRT estáticas compatibles; mezclar `/MD` y `/MT` en un mismo binario no es válido.
- Use `RUSTFLAGS` / `.cargo/config.toml` para `rustflags` con `-C target-feature=+crt-static` donde se admita, y verifique con Dependency Walker / `dumpbin /dependents`.

## macOS

- El modelo de plataforma de Apple espera **enlace dinámico** a *frameworks* del sistema; un binario gráfico «totalmente estático» **no** es lo habitual.
- **Alternativa para kits forenses:** empaquetar un **bundle de aplicación** con `@rpath` / `@loader_path` para `.dylib` incluidas, con firma de código si la política lo exige.

## Comandos de verificación sugeridos

```bash
cargo build --release
cargo build --release --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/foruster-desktop
```

Si `ldd` lista bibliotecas compartidas inesperadas, repita la auditoría para esa configuración de publicación y documente el resultado en las notas de versión.
