# Hash databases (NSRL and alert lists)

Foruster can compare each scanned file’s **MD5** and **SHA-1** digests against configurable sources:

- **Known-system / commercial software** — typically the [NIST National Software Reference Library (NSRL)](https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds) **RDSv3** publication in SQLite form. This is the standard public dataset for “known good” application and OS files used to reduce noise in forensic triage.
- **Alert lists (two tiers)** — **Suspicious** and **Evidence**, supplied by your organization as plain-text hash lists or SQLite databases using the same `FILE`-style schema as RDSv3. Foruster does **not** ship third-party illegal-content hash sets; lawful handling of such material remains the responsibility of the deploying organization.

### Why not a “police / Guardia Civil” public download?

National police forces, the Guardia Civil, and similar bodies **do not** publish operational hash databases for unrestricted public download in the same way [NIST publishes NSRL](https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds). Many alert categories are **legally restricted**, subject to **memoranda of understanding**, or distributed only through **controlled law-enforcement channels** — not as anonymous HTTP links inside a generic installer. Foruster therefore keeps these tiers **organization-supplied**: you add approved lists under `data/hashsets/` per your unit’s procedure, or start from empty placeholders.

## Privacy and security

- Only **hashes** are compared; nothing is uploaded by this feature.
- SQLite sources are opened **read-only**. Plain lists are loaded into memory; RDSv3 SQLite is **queried per file** so full datasets do not need to reside in RAM.

## Offline examination (typical forensic use)

- **During analysis**, Foruster does **not** require Internet access. Hash lookups use only files under your portable `data/hashsets/` tree (e.g. on USB).
- **Optional** large downloads (e.g. NIST curated demo) happen **only in the installer**, on a **preparation** machine that is allowed to reach the network. You then copy the finished installation to an **air-gapped** examination environment if your procedure requires it.

## Supported formats

| Kind | Extensions | Behaviour |
|------|------------|-----------|
| NSRL RDSv3 | `.sqlite`, `.db` | Detects a table with `MD5` and `SHA-1` / `SHA1` columns (e.g. `FILE`) and runs `SELECT 1 … WHERE lower(hash)=?`. |
| Plain list | `.txt`, other | One **hex** digest per line: **32** chars = MD5, **40** chars = SHA-1. Lines starting with `#` are comments. |

## Portable storage (forensic / USB)

All hash-set files and `hashsets-config.json` live under **`data/hashsets/` next to the Foruster
binary** (the deployment tree you copy to USB). **Nothing** is written under the host user profile
(`~/.local/share`, `%APPDATA%`, etc.).

- Override the root with absolute path `FORUSTER_PORTABLE_ROOT` if your layout differs (e.g. `bin/`
  and `data/` are not sibling folders).
- On-disk layout is defined by the `hashsets-config.json` schema (portable hash-set configuration).

### Installer

The graphical **foruster-installer** (run only on the preparation workstation, not on the
examined machine) configures **three tiers** independently. See [Installation and kit preparation](INSTALLER.md) for full behaviour (offline vs online mode, progress, logging).

**Known-good (NSRL-style)**

| Option | Behaviour |
|--------|-----------|
| **None** | No known-good SQLite file in the config (other tiers may still be set). |
| **Minimal sample** | Copies the embedded ~8 KiB RDSv3 SQLite (`known_system.sqlite`) — works **offline**. |
| **NIST curated demo** | Downloads NIST’s public RDSv3 curated demo zip (~87 MiB). Requires **online** mode in the installer **on this preparation PC only**. |
| **NIST curated legacy demo** | Second curated zip from an older NIST path; requires **online**. If NIST removes the URL, use another option or add a file manually. |
| **Full NSRL (Android / Legacy / Modern minimal, or Modern complete)** | Downloads an **official** multi-gigabyte RDSv3 zip from NIST’s S3 bucket (pinned release in the installer source, currently **2026.03.1**). Requires **online** mode, a **confirmation** dialog, and a **free-space** check (roughly **2×** the published download size). The NSRL is **not** generated locally — only **published** by NIST. |

**Suspicious** and **evidence** alert lists

| Option | Behaviour |
|--------|-----------|
| **None** | No path for that tier. |
| **Empty placeholder (.txt)** | Creates `alert_suspicious.txt` or `alert_evidence.txt` with comments only — add organization hashes later (same plain-text format as in [Supported formats](#supported-formats)). |
| **Demo sample lines** | Small `.txt` with public test-vector lines to verify parsing; replace for production use. |

If nothing is selected for any tier, the installer writes `hashsets-config.json` with lookups **disabled**.

After installation, run Foruster from that folder on the examination kit; it loads `data/hashsets/hashsets-config.json` **without any network**.

### First run without the installer

If there is no `hashsets-config.json` yet, Foruster may seed a **minimal** sample **only** under
`data/hashsets/` beside the executable (same portable rule). Disable with `FORUSTER_NO_DEFAULT_HASHSETS=1`.

## Runtime configuration

The desktop **Settings** page configures:

- Master enable/disable for hash lookups.
- **Hide known-good matches from profile results** — when a file matches a **known-good** source (e.g. NSRL), profile match events for that path can be suppressed so the match list focuses on user-relevant content.
- Paths for known-good, suspicious-alert, and evidence-alert sources.
- **Apply** reloads all sources from disk without restarting the application.

## References

- NIST — *Current RDS Hash Sets* (RDSv3): https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds  
- Forensics Wiki — NSRL overview: https://forensics.wiki/national_software_reference_library/

---

[Application home & downloads](https://m4rz3r0.github.io/foruster/)
