# Frequently asked questions

## What is Foruster?

Foruster is a cross-platform desktop application for **live-system forensic triage**: storage scanning, profile-based inspection, cryptographic hashes, PDF reporting, and an optional **WebAssembly** extension model with host-side sandboxing.

---

## Do I need a GitHub account to download?

**No.** Use the **[official project site](https://m4rz3r0.github.io/foruster/)** “Downloads” section: it lists files from the latest public release. You only need a web browser.

---

## Do I need command-line skills?

**Not for the basics.** You can work with the **file manager** and the **application window** alone. A terminal is only needed if your workflow requires advanced options (for example environment variables described under [Forensic policy and portable mode](FORENSIC_POLICY.md)).

---

## Which file should I choose?

- **Windows** — Prefer the **offline bundle** (`.zip`) if you want the app, models, and sample extensions in one folder. Use the **installer** (`.exe` name contains `installer`) to prepare a portable tree on removable media. The plain **`foruster-…-windows-x64.exe`** is the application executable for normal use.
- **Linux** — Same idea: **bundle** (`.tar.gz`) for an offline folder layout, **installer** binary for the graphical deployment tool, or the standalone **`foruster-…-linux-x64`** binary.

Verify integrity with **`SHA256SUMS`** from the same release when you need checksum confirmation.

---

## Is the application open source?

**No.** Foruster is **proprietary** software. Exact terms ship with your download and are linked from the **[project site](https://m4rz3r0.github.io/foruster/)**.

---

## Where is the installation guide?

Step-by-step help for the graphical installer is under [Installation and kit preparation](INSTALLER.md). The [documentation home](README.md) links all other topics (policy, hash databases, index).

---

## Can I run a USB kit on another PC?

That is the intended **portable** layout: the kit should contain the application and `data/` as prepared with the [installer](INSTALLER.md). Always follow your organization’s policy and the [Forensic policy and portable mode](FORENSIC_POLICY.md) document for **where** the tool may run.

---

## Does the program upload my files to the Internet?

**Not** during typical analysis: hash comparisons and core work are **local**. **Large downloads** (e.g. NSRL sets) are performed **through the installer** on the preparation workstation, not on the examined machine. See [Hash databases (NSRL and alert lists)](HASH_SETS.md) for detail.

---

[Application home & downloads](https://m4rz3r0.github.io/foruster/)
