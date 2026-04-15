# Forensic operation policy

Foruster is used on **live systems** under investigation. Investigators must control **where** the tool reads and writes. This document defines product behaviour and maps disk interactions to risk levels.

## Definitions

| Term | Meaning |
|------|---------|
| **Analyzed volume** | Any filesystem path, disk, or partition added for analysis (evidence). |
| **Portable tree** | Directory containing the Foruster binary and optional `data/` (USB kit, network share prepared for the case, etc.). Set explicitly with `FORUSTER_PORTABLE_ROOT` when the layout differs from “binary beside `data/`”. |
| **Host OS profile** | OS-specific user config (e.g. `%APPDATA%`, `~/.config`) — **not** part of the portable kit. |
| **User-directed export** | Saving only through a **Save** / **Export** dialog where the investigator chooses the destination. |

## Policy matrix

| Area | Goal |
|------|------|
| **Analyzed volumes** | Read-only from Foruster’s perspective: no creating, modifying, or deleting evidence files. Analysis uses directory traversal and file reads; plugins receive registered file handles only. |
| **Host system disk** | Minimise silent writes. Prefer the **portable tree** for configuration, models, hash sets, and scratch data when **forensic mode** is enabled. |
| **Exports (reports, CSV, JSON, images)** | Allowed only as **user-directed exports** to paths chosen in the UI — never silent writes to arbitrary locations. |

## Default behaviour (no configuration required)

**Forensic mode is on by default** when the application starts (double-click or command line with no flags). No environment variables are required for legal or forensic audits: preferences and scratch data go under the portable `data/` tree next to the binary (or under `FORUSTER_PORTABLE_ROOT`), not under the host user profile or system temp.

To use the **standard** layout (OS user profile, system temp directory, optional automatic hash-set seeding when no config exists), start with:

- **`--standard`** or **`--no-forensic`** on the command line, or  
- **`FORUSTER_FORENSIC_MODE=0`** (or `false` / `no`), or  
- **`FORUSTER_STANDARD_MODE=1`** only when `FORUSTER_FORENSIC_MODE` is not set to a non-empty value.

Run **`foruster-desktop --help`** for a short summary (from a terminal; on Windows, GUI builds may not show a console when launched from Explorer).

**Precedence:** command-line `--forensic` / `--standard` / `--no-forensic` overrides `FORUSTER_FORENSIC_MODE` and `FORUSTER_STANDARD_MODE` when present.

## Environment variables

| Variable | Effect |
|----------|--------|
| `FORUSTER_PORTABLE_ROOT` | Absolute path to the root of the portable deployment (overrides “directory of current executable”). Writable `data/` is under this root. |
| `FORUSTER_FORENSIC_MODE` | If **unset** or **empty**, forensic mode is **on** unless `FORUSTER_STANDARD_MODE` opts out (see above). If set to a non-empty value: `0` / `false` / `no` → standard mode; any other value → forensic mode. |
| `FORUSTER_STANDARD_MODE` | When set to a truthy value and `FORUSTER_FORENSIC_MODE` is unset or empty → standard mode. Ignored when `FORUSTER_FORENSIC_MODE` is explicitly non-empty. |
| `FORUSTER_NO_DEFAULT_HASHSETS` | If set, skips portable hash-set loading and seeding entirely (legacy opt-out). |

## Disk I/O inventory (application components)

Operations are **read (R)** or **write (W)**. “Risk” is impact on the **analyzed system** if the app is run from/installs on that system without a portable kit.

| Component / operation | Target volume | R/W | Risk | Mitigation |
|----------------------|---------------|-----|------|------------|
| Analysis walker / hashing | Analyzed paths | R | Low if read-only APIs used | Linux: `O_NOATIME` when permitted; see [Windows file access](#windows-file-access-and-timestamps). |
| Plugin host: `read_file`, `compute_hash`, `decode_image`, `run_inference` | Paths passed as registered `FileEntry` | R | Low | Per-invocation read budgets; WASM sandbox (no arbitrary paths). |
| Plugin host: `query_sqlite` | Copy of DB to scratch | W (scratch only) | Medium if scratch is OS temp | Forensic mode: copies under portable `data/scratch/`. |
| Desktop: UI preferences | Host profile **or** `data/config/` | W | Medium (host profile) | Forensic mode writes under portable tree only. |
| Desktop: portable hash sets | `data/hashsets/` | R/W | Low if kit is on removable media | Forensic mode: no automatic seed; optional `FORUSTER_NO_DEFAULT_HASHSETS`. |
| Desktop: PDF / JSON / CSV / image export | User-chosen path | W | User-controlled | Save dialogs only. |
| Desktop: “Open folder” after export | OS opens explorer | Indirect | User environment | Investigator chooses export location. |
| Desktop: browser cookie helper | Copies DB to scratch, launches browser | W (scratch) | Medium | Forensic mode: scratch under portable tree. |
| Installer / `foruster-installer` | Download / unpack paths | W | **Preparation tool** | Not intended for execution on the analyzed machine during acquisition; use to build the kit on a separate system. |

## Windows file access and timestamps

- **Linux** uses `O_NOATIME` when opening files for format sniffing where the kernel and permissions allow, with fallback to normal open if `EPERM` (see `crates/analysis/src/utils.rs`).
- **Windows** does not expose a direct `O_NOATIME` equivalent in the standard library. Last-access time behaviour depends on **NTFS policy** (e.g. last access updates may be disabled system-wide on recent Windows versions for performance). Opening files for read may still update metadata depending on OS settings.
- **Recommendation:** Run the tool from a **portable medium** with `FORUSTER_FORENSIC_MODE` set, document the **host OS version** and **filesystem**, and treat timestamp preservation as **best-effort** on Windows unless using specialised drivers or hardware write blockers at the volume level.

## Related documentation

### Operator guides (this folder)

- [Hash databases (NSRL and alert lists)](HASH_SETS.md) — layout under `data/hashsets/`.
- [Installation and kit preparation](INSTALLER.md) — preparation workstation and kit layout.
- [Topic index](INDEX.md) — documentation map for operators.

---

[Application home & downloads](https://m4rz3r0.github.io/foruster/)
