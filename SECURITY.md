# Security Policy

## Overview

Foruster is a forensic tool intended for use in legal and judicial contexts. The integrity and trustworthiness of the Software are of paramount importance. We take security vulnerabilities seriously and appreciate responsible disclosure by the community.

---

## Supported Versions

Security updates are applied to the **latest stable release** only. We strongly recommend always using the most recent version of Foruster available on the [Releases](https://github.com/m4rz3r0/foruster/releases) page.

| Version        | Supported          |
|----------------|--------------------|
| Latest release | :white_check_mark: |
| Older releases | :x:                |

---

## Reporting a Vulnerability

> **DO NOT open a public GitHub Issue to report a security vulnerability.**

Because Foruster is used in digital forensic investigations and legal proceedings, public disclosure of security vulnerabilities before a patch is available could have serious consequences. We therefore require all security-related reports to be submitted through **private channels**.

### How to Report

Please send a detailed vulnerability report via email to:

**security [at] foruster [dot] io**

_(Replace [at] with @ and [dot] with . when composing the email.)_

Your report should include, where applicable:

- A clear description of the vulnerability and its potential impact.
- The version(s) of Foruster affected.
- The operating system and environment in which the vulnerability was observed.
- Step-by-step instructions to reproduce the issue.
- Any proof-of-concept code, screenshots, or supporting materials.
- Your suggested remediation, if any.

We will acknowledge receipt of your report within **72 hours** and will work with you to assess, validate, and address the issue in a timely and coordinated manner.

---

## Coordinated Disclosure

We follow a **responsible coordinated disclosure** model. We ask that reporters:

- Allow us a reasonable time to investigate and release a patch before any public disclosure.
- Avoid exploiting the vulnerability beyond what is strictly necessary to demonstrate its existence.
- Refrain from accessing, modifying, or deleting data that does not belong to you.

We commit to keeping reporters informed of the remediation progress and to crediting responsible reporters (with their consent) in the release notes.

---

## Binary Signing

All official Foruster release binaries are **digitally signed** by the Foruster Team. Verifying the digital signature before executing the binary is strongly recommended, particularly in forensic and legal contexts, as it provides an additional layer of assurance regarding the authenticity and integrity of the Software.

Refer to the [README.md](./README.md#integrity--chain-of-custody) for instructions on verifying SHA-256 checksums included with each release.

---

## Scope

This security policy applies to vulnerabilities in the Foruster binary distributions. Reports concerning third-party dependencies, underlying operating system components, or general usage questions are out of scope for this policy.
