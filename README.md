<div align="center">
  <h1>Foruster</h1>
  <p align="center"><strong>Live forensic triage and anomaly detection</strong></p>
  <p align="center">
    <a href="https://slint.dev">
      <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo/MadeWithSlint-logo-light.svg" height="48">
    </a>
  </p>
</div>

---

**Languages:** [English](#documentation) · [Español](README.es.md)

This repository is the **public documentation and release channel** for **Foruster**. **Application source code is not published here.** Canonical development happens on a private forge; this space is limited to docs that are safe to share and binaries attached to [Releases](https://github.com/m4rz3r0/foruster/releases).

## Documentation

| Topic | Document |
|------|----------|
| Index | [doc/INDEX.md](doc/INDEX.md) |
| WASM plugin SDK (host API, ABI) | [doc/PLUGIN_SDK.md](doc/PLUGIN_SDK.md) |
| Plugin authoring tutorial | [doc/PLUGIN_DEVELOPMENT_GUIDE.md](doc/PLUGIN_DEVELOPMENT_GUIDE.md) |
| Forensic / portable behaviour | [doc/FORENSIC_POLICY.md](doc/FORENSIC_POLICY.md) |
| Static builds | [doc/STATIC_BUILDS.md](doc/STATIC_BUILDS.md) |
| Hash sets (NSRL-style) | [doc/HASH_SETS.md](doc/HASH_SETS.md) |
| Installer | [doc/INSTALLER.md](doc/INSTALLER.md) |
| Verify WASM plugins | [doc/PLUGIN_BUILD_VERIFY.md](doc/PLUGIN_BUILD_VERIFY.md) |

These files are **redacted for public distribution**: they omit internal repository layout, host implementation paths, and internal-only workflows. Some links may still point at paths that exist only in a full SDK or partner drop (for example under `examples/`).

## Product

Foruster is a cross-platform desktop application for **live-system forensic analysis**: scanning active storage, profile-based search, cryptographic hashes, PDF reporting, and a **WebAssembly** extension model with a host-side sandbox.

The UI is built with [Slint](https://slint.dev/) (see attribution above).

## License

**Foruster** is **proprietary, closed-source software** as distributed to licensees or customers. Documentation in this repository is published for reference; third-party components remain under their respective licenses. Do not infer from this README alone how any particular binary build is licensed; check the notice shipped with that build.
