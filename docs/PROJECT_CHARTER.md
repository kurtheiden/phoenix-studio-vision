# Project Charter

## Purpose

Phoenix is an open-source, exploratory reverse-engineering project. Its purpose
is to build evidence-based tools for safely inspecting and, over time,
understanding binary files associated with Studio Vision.

## Current scope

The initial command-line tool performs format-neutral, read-only inspection:

- report a file's name, canonical path, and byte length;
- calculate its SHA-256 digest; and
- display at most the first 256 bytes as hexadecimal and ASCII.

## Principles

- Preserve source material: input files are never modified.
- Separate observations from hypotheses.
- Do not invent signatures, structures, or format claims.
- Prefer small, testable milestones over premature architecture.
- Do not publish samples without clear redistribution rights.
- Treat all input as untrusted.
- Never substitute instruments automatically without user approval.

## Out of scope for the initial milestone

- Studio Vision file detection or parsing
- File conversion, repair, or modification
- A graphical application or foreign-function interface
- Claims of compatibility with undocumented formats
