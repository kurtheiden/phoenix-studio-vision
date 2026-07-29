# Authentic Studio Vision Project Samples

Authentic Studio Vision and Studio Vision Pro project samples are maintained
separately from the Phoenix source code. They are research evidence, not source
files, test fixtures, or application assets. Keeping them separate reduces the
risk that development tools or source-control operations will modify them and
keeps the repository focused on redistributable Phoenix code and documentation.

## Why samples are excluded from Git

Authentic projects may contain copyrighted, personal, confidential, or
otherwise non-redistributable material. They can also carry legacy filesystem
metadata that Git does not preserve. Authentic samples must therefore remain
outside the repository and must not be committed. Phoenix documentation may
record evidence observed in those files without distributing the files
themselves.

## Sample identifiers

Research documents may assign a stable local identifier to a sample, such as
`project-001`. An identifier allows observations to be cross-referenced
without depending on a sensitive or ambiguous filename. The research record
should associate the identifier with the sample's exact filename, byte size,
and SHA-256 digest. Assigning an identifier does not rename or modify the
sample file.

## Metadata preservation

Finder metadata and extended attributes should be preserved when they are
available. This includes Finder Type and Creator codes and any other attributes
present on the inspected copy. Copying a sample through filesystems or tools
that do not preserve this metadata can remove evidence. Research notes must
state which metadata was directly available on the inspected copy and must not
assume that absent metadata was absent from the original file.

## Evidence categories

All research observations must clearly distinguish among:

- Directly observed facts: measurements, bytes, strings, metadata, prompts, or
  behavior observed in a specific inspection or test.
- Documented Opcode behavior: statements supported by identified original
  Opcode documentation.
- Hypotheses: proposed explanations or interpretations that remain unconfirmed.

Research must remain evidence-based. Hypotheses must not be presented as facts,
and observations from one sample must not be generalized to all Studio Vision
projects without sufficient supporting evidence.

## Expected workflow

1. Copy an authentic project into the external research directory without
   changing its filename.
2. Compute the copy's SHA-256 digest.
3. Record Finder metadata and extended attributes when available.
4. Perform read-only inspection on the copy.
5. Document observations in the Phoenix documentation with the appropriate
   evidence category and sample identifier.
6. Never modify the sample.

Original project files should be preserved elsewhere whenever possible.
Phoenix and research tools must inspect copies only. Discoveries belong in the
Phoenix documentation, never in changes to the authentic sample files.
