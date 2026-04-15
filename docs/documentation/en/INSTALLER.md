# Installation and kit preparation (`foruster-installer`)

The **`foruster-installer`** utility is a graphical tool that **prepares a deployment kit**: it copies the Foruster application and optional assets (hash sets, models, etc.) onto a **destination medium** (for example a USB stick prepared on a lab workstation).

!!! danger "Do not run the installer on the examined system"
    Use it **only on the preparation workstation**. Running it on the machine under examination can trigger downloads, writes, or configuration changes that affect **chain of custody**.

!!! tip "Who this guide is for"
    If you only need to **unpack a downloaded bundle** (a folder that already contains the app), extracting the archive may be enough. This page covers the **graphical installer** when you want guided choices for **releases**, **hash databases**, and **destination paths**.

---

## At a glance: what you are doing

| Phase | What happens |
|-------|----------------|
| 1. Source | Choose **local files** (no network) or **downloads** (network on this machine only). |
| 2. Versions & extensions | In online mode, pick **which application and extension releases** to install. |
| 3. Destination | Select the **folder or drive** that will hold the portable tree (for example the USB volume). |
| 4. Optional hashes | Decide which **NSRL** and **alert list** content is copied or downloaded under `data/hashsets/`. |

When finished, you have a **portable folder** you can run from the intended host, per your procedure.

---

## Before you start (short checklist)

- **Disk space**: full NSRL downloads need **many gigabytes** free on the destination; the installer performs approximate free-space checks.
- **Network**: **online** mode should be used only on the **preparation** machine where your policy allows outbound access.
- **Local bundle**: in **offline** mode, place the **bundle** next to the installer (`foruster` / `foruster.exe` and supporting folders), or set `FORUSTER_BUNDLE_ROOT` if the bundle lives elsewhere.

---

## Source modes

| Mode | Use case |
|------|----------|
| **Offline** | The installer sits next to a local **bundle** (`foruster` / `foruster.exe` and supporting folders). Nothing is downloaded. |
| **Online** | Fetches release bundles and extension manifests from GitHub (TLS). Requires network **only on this preparation PC**. |

Set `FORUSTER_BUNDLE_ROOT` to point at a bundle directory if it is not next to the installer executable.

---

## Versions (online mode)

This block is **always visible**. In **Offline** mode its controls are **disabled** (and the card is visually de-emphasized) so switching SOURCE does **not** move the rest of the form — better for accessibility and predictable layout. When **SOURCE** is **Online**, enable **Show version selection** to pick:

- **Application release** — Tags that publish a platform bundle for the selected OS (or **Latest** for the default release).
- **Extensions release** — A `plugins/…` tag for `plugins-manifest.json` (or **Latest plugins**).

Use **Apply** next to the extensions list to reload the manifest after changing the extensions tag. **Refresh version lists** re-queries for tags (e.g. after a new release).

---

## Target platform

Choose **Linux** or **Windows** so the correct binary name is installed on the destination tree. Changing the target refreshes the **application** tag list when online.

---

## Hash databases (`data/hashsets/`)

The installer writes everything under the chosen **destination folder** — portable layout: `data/hashsets/` next to the app. Nothing is stored under the host user profile.

You can configure **three independent tiers**:

1. **Known-good (NSRL / catalogued software)**  
   - **None** — no known-good SQLite in the config (other tiers may still be set).  
   - **Minimal sample** — embedded small RDSv3 SQLite (~8 KiB), works **offline**.  
   - **NIST curated demo** — downloads NIST’s public RDSv3 curated zip (~87 MiB). Requires **online** mode.  
   - **NIST curated legacy demo** — second curated zip from an older NIST layout; requires **online**. If the URL is retired by NIST, pick another option or place a file manually.  
   - **Full NSRL publications** — **Android minimal**, **Legacy minimal**, **Modern minimal**, or **Modern complete** RDSv3 zips from NIST (multi‑GB; release version is pinned in the installer). Requires **online** mode, a **confirmation** dialog, and enough **free disk space** (the installer checks roughly **2×** the expected download size). These are **official NIST downloads**, not files generated on your machine.

2. **Suspicious-tier alert list**  
   - **None** — no file.  
   - **Empty placeholder (.txt)** — creates `alert_suspicious.txt` with comments only.  
   - **Demo sample lines** — small `.txt` with public test-vector lines so you can verify parsing; replace for production.  
   There is **no** public law-enforcement hash feed to download here like NIST’s NSRL; operational lists are organization-supplied (see [Hash databases (NSRL and alert lists)](HASH_SETS.md)).

3. **Evidence-tier alert list**  
   - **None**, **Empty placeholder**, or **Demo sample lines** — same idea as suspicious (`alert_evidence.txt`). Same note as above for law-enforcement sources.

If every tier is effectively empty, the installer writes a **disabled** `hashsets-config.json` (lookups off).

---

## Progress, log, and completion

- While installing, a **progress bar** reflects download/extract/copy and hash-database steps (coarse milestones).  
- Text lines are appended to the on-screen **log**; the same messages are emitted with **`tracing`** (use `RUST_LOG`, e.g. `RUST_LOG=info`).  
- When the run finishes, a **native dialog** reports success or failure (short message; details remain in the log).

The installer UI is available in **English**, **Spanish**, and **French** (language buttons on the disclaimer and main screen).

---

## Analysis vs preparation

The **Foruster application** does not need the Internet **during analysis**. Hash lookups use only files under your portable `data/hashsets/` tree. Optional large downloads happen **in the installer**, on the preparation workstation; conceptual detail is in [Hash databases (NSRL and alert lists)](HASH_SETS.md).

---

## More help

- [Frequently asked questions](FAQ.md) — which file to download and common issues.  
- [Forensic policy and portable mode](FORENSIC_POLICY.md) — where the app writes in forensic mode.

---

[Application home & downloads](https://m4rz3r0.github.io/foruster/)
