# Controlled Save Experiments

Phoenix uses controlled save experiments to identify which binary regions
change when exactly one known user action is performed in Studio Vision. These
experiments provide comparative evidence; they do not, by themselves, explain
Studio Vision's internal file format.

The experiment files are maintained outside the repository in the external
Controlled Save Experiments collection. Phoenix inspects every file in that
collection read-only.

## Methodology

Every controlled save experiment follows these principles:

- Every experiment begins with the same original authentic Studio Vision
  project, `newest STUFF`.
- The original project is never modified.
- Each experiment is performed independently.
- Only one user-visible variable is changed in each experiment, unless the
  experiment explicitly tests **Save As** with no project edits.
- The resulting project is always saved under a new filename.
- Phoenix performs read-only inspection of the original and resulting files.
- Experimental records distinguish direct measurements from hypotheses.

Direct measurements may include file sizes, cryptographic digests, Finder
metadata, resource-fork observations, changed byte offsets, and printable
string differences. Any proposed interpretation of those measurements must be
identified as a hypothesis requiring further evidence.

## Experimental environment

- Application: StudioVision PPC
- Execution environment: SheepShaver
- Original project: `newest STUFF`

## Current experiment series

### Experiment 001 — Track Name Change

- Source: `newest STUFF`
- Change: track name `Track 7` changed to `Track 7 TEST`

### Experiment 002 — Save As with No Edits

- Source: `newest STUFF`
- No project edits
- Saved under a new filename

### Experiment 003 — Change One Instrument Assignment

- Source: `newest STUFF`
- Change: one track instrument assignment changed

### Experiment 004 — Change One Tempo Value

- Source: `newest STUFF`
- Change: tempo changed from 120 BPM to 130 BPM

## Future experiments

Future experiments should continue to:

- change exactly one variable whenever possible;
- preserve the untouched original;
- save the experimental result under a new filename;
- document the exact user-visible change; and
- avoid making multiple simultaneous edits.

Following this discipline keeps comparisons reproducible and helps separate
directly observed changes from interpretations that require additional
controlled experiments.
