[← Application home & downloads](https://m4rz3r0.github.io/foruster/)

# Foruster — Documentation

Foruster is an **intelligent forensic system** for **digital forensic triage** on **live systems**: it helps prioritize what to inspect *in situ* before or in addition to exhaustive laboratory work. It integrates **computer vision** and **deep learning** through **on-device inference** — **without sending digital evidence** to cloud services.

---

## Who this is for

These pages are written for **people who need to run the tool responsibly** even if they **do not work with advanced IT daily**. You **do not need** a GitHub account or command-line skills to **download** and **prepare a kit** in typical scenarios.

!!! tip "Two different machines"
    In forensic workflows it helps to separate: a **preparation workstation** (downloads, USB preparation, etc.) and the **system or medium under examination** (where only the prepared kit should run, per your organization’s policy).

---

## How to get Foruster

1. Open the **[project home page](https://m4rz3r0.github.io/foruster/)** in a normal web browser.
2. Go to the **Downloads** section.
3. Pick the file that matches your platform (**Windows** or **Linux**). The [FAQ](FAQ.md) explains bundles, the installer, and standalone binaries.
4. **No account** is required on GitHub or elsewhere to download.

**License terms** ship with your download and are linked from the same site.

---

## Main guides

| Guide | What it covers |
|-------|----------------|
| [Installation and kit preparation](INSTALLER.md) | The graphical installer, offline/online modes, and what lands on the destination medium. |
| [Forensic policy and portable mode](FORENSIC_POLICY.md) | Disk read/write behaviour, chain of custody, and environment variables. |
| [Hash databases (NSRL and alert lists)](HASH_SETS.md) | Hash sets, alert tiers, and `data/hashsets/` configuration. |
| [Frequently asked questions](FAQ.md) | Downloads, licensing, file choice, and first steps. |
| [Topic index](INDEX.md) | A compact map of all topics. |
