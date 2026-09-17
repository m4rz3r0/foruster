# Foruster

**Forensic acquisition and analysis of storage.**

A desktop tool for judicial-police units and court-appointed examiners. It
acquires digital evidence from a running system without modifying it, and
leaves verifiable proof of what was collected, when, by whom, and with what
integrity.

*English · [Español](README.md)*

---

## Purpose

In legal proceedings it is not enough to find a file: you have to be able to
show it is the same file that was on the device, and that nobody altered it
along the way. Foruster is built around that requirement.

The principle behind everything: **nothing is reported that was not first
acquired, hashed and recorded in an auditable container.**

## What it does

**Acquisition with chain of custody.** Every item is read read-only, hashed
with SHA-256 as it is copied, and recorded in a manifest. The audit log is
hash-chained, so any later alteration is detectable. Custody transfers are
appended to that same chain, and re-verification proves it was not broken.

**Case identity is mandatory.** Without a case number, organisation, operator
and operator ID, acquisition does not start. Those details live inside the
container, not in the interface.

**Volatile memory.** Acquires the physical memory of a running machine without
installing anything on it, in a standard format third-party analysis tools read
directly. The kernel context needed to interpret the image travels with it — and
it is gone once the machine is powered off.

**Targeted triage.** Rather than cloning a whole disk, it locates and collects
specific artifacts from a catalogue — browser cookies and history among them —
under the same guarantees. Databases always travel with their sidecar files, so
an incomplete copy is never acquired unknowingly.

**Manifest signing.** Ed25519 signature using the laboratory's own key.
Verifiable by a third party with standard tooling, without running Foruster: an
opposing expert can check the work independently.

**Deterministic reporting.** The same container always produces the same
report, in text and PDF. Nothing is recomputed at export time.

**Honest about what is incomplete.** A failed item marks the session partial, an
unimplemented capability says so, and known limitations of the analysis are
declared in the report rather than omitted.

## How it is used

Three interfaces over one forensic core:

| | For |
|---|---|
| **Graphical** | Everyday laboratory work and fieldwork. |
| **Terminal** | Machines with no desktop environment. Single binary, no install. |
| **Command line** | Automation and integration with laboratory systems. JSON output. |

Typical flow: select targets → acquire with the case details → verify → sign →
export the expert report.

Runs on Linux and Windows. The terminal and command-line interfaces ship as a
static binary, suitable for a portable kit.

## What it does not do

Stated plainly, because it matters before citing the tool in an expert report:

- **Not homologated, certified or officially validated** by any body.
- **No qualified signature** in the eIDAS sense, and no timestamping: its time
  mark does not constitute a trusted date.
- **No memory acquisition on Windows**, and no breaking of operating-system
  encryption.
- **The memory image is not a snapshot of a single instant**: the machine keeps
  running while it is copied.
- **No CSAM detection.**
- **Does not modify** the system under examination.

## Availability

Product under development. For demonstrations, evaluation or licensing terms,
please use the contact address on the profile.

## Licence

Proprietary software. All rights reserved. See [`LICENSE`](LICENSE).

The source code is not published. This repository contains product information
only.
