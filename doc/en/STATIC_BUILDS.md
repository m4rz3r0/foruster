# Static and portable binary builds

Goal: ship executables that depend **only** on the OS executable format and kernel interfaces, avoiding separate copies of `libc`, MSVC runtime, or other DLLs where practical.

## Linux: musl and fully static linking

- **Target:** `x86_64-unknown-linux-musl` (or `aarch64-unknown-linux-musl` on Apple Silicon build hosts with cross-toolchains).
- **Rust:** `rustup target add x86_64-unknown-linux-musl` and install a musl linker (e.g. `musl-gcc` / distro packages, or `cross` from `cross-rs`).
- **Typical flags:** `RUSTFLAGS='-C target-feature=+crt-static'` produces a static executable when all native dependencies support it.

### Limitations (source builds)

| Dependency | Notes |
|------------|--------|
| **Slint** | Uses platform backends (e.g. X11/Wayland, fontconfig on many Linux desktops). A musl build may still need **system libraries** at runtime unless you use a fully bundled backend — verify with your Slint version and feature flags. |
| **`ort` (ONNX Runtime)** | The `ort` crate often loads **dynamic** ONNX Runtime libraries or bundles prebuilt `.so` files. **Fully static** linking of ONNX + Slint + GPU stacks is frequently **impractical**; treat “static Rust binary + separate `libonnxruntime.so`” as a realistic floor unless you build ONNX from source and link statically (advanced). |

### Offline / no-download builds (`ort`)

A typical source checkout pins `ort` with **`download-binaries`** so `cargo build` can fetch ONNX Runtime when needed. That is convenient for development but may be undesirable for **air-gapped** or **reproducible** forensic kit builds.

Options:

1. **Vendor ONNX Runtime** — Install the official ONNX Runtime package for your platform (or copy `libonnxruntime.so` / `.dll` / `.dylib` from a trusted build) and point the loader at it. Adjust the `ort` **feature flags** in the root `Cargo.toml` to disable `download-binaries` and use the feature set your `ort` version documents for **system** or **manual** libraries (see the [`ort` crate docs](https://docs.rs/ort) for the exact feature names for your release).
2. **Environment variables** — Many `ort` / ONNX Runtime setups honour variables such as `ORT_DYLIB_PATH` (name may vary by version) to load a specific shared library; set this in the build or runtime environment after placing the library under your portable tree.
3. **Document the artifact** — For each release, record the ONNX Runtime version and path next to the binary (see [FORENSIC_POLICY.md](FORENSIC_POLICY.md) for portable layout).

After changing `ort` features, run a full `cargo build --release` on a clean machine **without** network access to confirm nothing is fetched at compile time.
| **Font / UI stack** | Even with musl, `fontconfig`, `freetype`, or similar may be required at runtime unless the UI stack is built to embed or avoid them. |

**Recommendation:** Document the **actual** `ldd` output (or equivalent) for each release artifact. Prefer “**mostly static** Rust + documented `.so` list” over promising a single file with zero dependencies unless you verify it.

## Windows: MSVC runtime

- Default **MSVC** builds link against the **Universal C Runtime** and may require **VC++ Redistributable** on the target machine unless you link with **`/MT`** (static CRT) for the Rust toolchain and all C++ dependencies.
- **Slint** and **Win32** dependencies must be built with compatible static CRT settings; mixing `/MD` and `/MT` in one binary is invalid.
- Use `RUSTFLAGS` / `.cargo/config.toml` to set `rustflags` for `-C target-feature=+crt-static` where supported, and verify with Dependency Walker / `dumpbin /dependents`.

## macOS

- Apple’s platform model expects **dynamic linking** to system frameworks; a “fully static” GUI binary is **not** the normal approach.
- **Alternative for forensic kits:** ship an **app bundle** with `@rpath` / `@loader_path` for bundled `.dylib` dependencies, code-signed if required by policy.

## Suggested verification commands

```bash
# Linux GNU target (typical dev build)
cargo build --release

# Linux musl (after installing target + linker)
cargo build --release --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/foruster-desktop  # should show “statically linked” or list only what you expect
```

If `ldd` lists unexpected shared libraries, repeat the audit for that release configuration and document them in release notes.
