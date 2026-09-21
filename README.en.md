# Foruster

Forensic acquisition of running machines.

**[See the site](https://m4rz3r0.github.io/foruster/)** ·
[Documentation](https://m4rz3r0.github.io/foruster/documentacion/) ·
[Contact](https://m4rz3r0.github.io/foruster/#contacto)

*English · [Español](README.md)*

> The product ships in Spanish. This page is here so the repository can be read
> from outside Spain; the application, the site and the documentation are in
> Spanish only.

---

Foruster copies the memory and the storage of a running machine without modifying
it, and leaves a case file a third party can verify on their own.

Finding a file is not enough. You have to show it is the same file that was on the
device and that nobody touched it along the way. That is where the rule the rest
follows comes from: nothing is reported that was not first acquired, sealed with
its hash and recorded in the case file.

## How it works

Media are opened read-only. Every item is hashed with SHA-256 while it is copied,
not once copying ends, and recorded in a manifest. The audit log is hash-chained. Alter
the case file after it is closed and the chain stops adding up. Custody
transfers are appended to that same chain, and checking it again proves it is
still whole.

Without a case number, organisation, operator and operator ID, acquisition does
not start. Those details are written inside the case file, not left on screen.

Volatility sets the order, not convenience. First the physical memory of the
running machine, which is gone once it is switched off; the kernel context needed
to interpret the image travels with it. Then the storage you selected.

Cloning a whole disk is not a prerequisite for working. Foruster locates specific
artifacts from a catalogue — browser cookies and history among them — with the
same guarantees, and takes each database together with its sidecar files, so you
do not end up with an incomplete copy without knowing it.

The manifest is signed with Ed25519, using the laboratory's key. It verifies with
`openssl`, without running Foruster, so the other side can review the work without
taking any claim of ours on trust.

The same case file always yields the same report, in text and in PDF. Exporting
recalculates nothing, it just reads what was already stored.

And what went wrong is on the record. A failed item marks the session partial, a
capability that is not implemented says so rather than staying quiet, and the
known limits of the analysis are declared in the report itself.

## How it is used

Three interfaces. The graphical one, for laboratory work and fieldwork. The
terminal one, for machines with no desktop. And the command-line one, which emits
JSON and is there to automate.

There are two ways to start. **Explore** walks the mounted disks and classifies
what is there without copying anything. **Acquire** takes what you marked into a
case file. Looking before taking anything is the usual order. Verification, the
signature and the report come after, in that order.

It runs on Linux and Windows. The terminal and command-line interfaces are a
single static executable, suitable for a portable kit.

## What it does not do

- It is not homologated, certified or validated by any body.
- The signature is not qualified in the eIDAS sense, and there is no timestamping:
  its timestamp does not constitute a legally certain date.
- It does not acquire physical memory on Windows.
- The memory image is not a snapshot of a single instant: the machine keeps
  running while it is copied.
- It does not break operating-system encryption.
- It does not detect CSAM.
- It does not modify the machine under examination.

## Licence

Proprietary software. All rights reserved. See [`LICENSE`](LICENSE).

Get in touch [from the site](https://m4rz3r0.github.io/foruster/#contacto).
