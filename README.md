# Foruster — Digital Evidence Extraction & Analysis

[![Latest Release](https://img.shields.io/github/v/release/m4rz3r0/foruster?label=Release&style=flat-square)](https://github.com/m4rz3r0/foruster/releases/latest)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-blue?style=flat-square)](#installation)
[![License](https://img.shields.io/badge/License-Proprietary%20EULA-red?style=flat-square)](./LICENSE)

---

## Introduction

**Foruster** is a cross-platform desktop application for the extraction and analysis of digital evidence, designed to be used in the context of legal proceedings and judicial investigations. It provides investigators, forensic analysts, and legal professionals with a reliable, reproducible, and auditable workflow for acquiring and examining digital artifacts from computing systems.

Foruster is the evolution of a project previously published in the **SoftwareX** journal. The original work established the scientific and methodological foundation of the tool; this release represents a major advancement in its capabilities, usability, and cross-platform support.

> **Target Audience:** Digital forensics practitioners, law enforcement agencies, legal professionals, and academic researchers operating under applicable legal frameworks.

---

## Installation

No installation is required. Foruster is distributed as a self-contained binary.

1. Navigate to the [**Releases**](https://github.com/m4rz3r0/foruster/releases) tab of this repository.
2. Download the binary appropriate for your operating system:
   - `foruster-<version>-windows-x64.exe` — for Windows (64-bit)
   - `foruster-<version>-linux-x64` — for Linux (64-bit)
3. Make the binary executable (Linux only):
   ```bash
   chmod +x foruster-<version>-linux-x64
   ```
4. Run the application:
   ```bash
   # Linux
   ./foruster-<version>-linux-x64

   # Windows (PowerShell or Command Prompt)
   .\foruster-<version>-windows-x64.exe
   ```

---

## Integrity & Chain of Custody

Every release includes a `SHA256SUMS` file alongside the distributed binaries. This file contains the SHA-256 cryptographic hashes of all release artifacts.

Verifying the integrity of the downloaded binary is **strongly recommended** before use in any investigative or legal context, as it guarantees:

- The binary has not been tampered with or corrupted during download.
- The file originates from an official Foruster release.
- The chain of custody for the software itself is preserved.

**Verification (Linux):**
```bash
sha256sum --check SHA256SUMS
```

**Verification (Windows — PowerShell):**
```powershell
Get-FileHash .\foruster-<version>-windows-x64.exe -Algorithm SHA256
```
Compare the output against the corresponding entry in the `SHA256SUMS` file provided in the release.

Additionally, release binaries are **digitally signed**. Refer to [SECURITY.md](./SECURITY.md) for details on signature verification.

---

## Disclaimer

> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. THE FORUSTER TEAM MAKES NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, AND NON-INFRINGEMENT.
>
> THE USER IS SOLELY RESPONSIBLE FOR ENSURING THAT THE USE OF THIS TOOL COMPLIES WITH ALL APPLICABLE LOCAL, NATIONAL, AND INTERNATIONAL LAWS AND REGULATIONS, INCLUDING BUT NOT LIMITED TO LAWS GOVERNING DIGITAL INVESTIGATIONS, PRIVACY, DATA PROTECTION, AND THE ADMISSIBILITY OF DIGITAL EVIDENCE IN LEGAL PROCEEDINGS.
>
> THE FORUSTER TEAM SHALL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES ARISING FROM THE USE OR MISUSE OF THIS SOFTWARE.

By downloading and using Foruster, you acknowledge that you have read, understood, and agree to the terms set forth in the [LICENSE](./LICENSE) agreement.

---

## License

Foruster is currently distributed free of charge under a proprietary End User License Agreement (EULA). See [LICENSE](./LICENSE) for full terms. Reverse engineering, decompilation, disassembly, and modification of the binaries are strictly prohibited.

The Foruster Team reserves the right to introduce commercial licensing requirements for professional or institutional use in future versions of the Software.

---

## Contributing & Bug Reports

Since the source code is closed, we do not accept Pull Requests. However, we actively welcome detailed issue reports and feature suggestions. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidance.

## Security

To report a security vulnerability, please **do not** open a public issue. Follow the private disclosure process described in [SECURITY.md](./SECURITY.md).