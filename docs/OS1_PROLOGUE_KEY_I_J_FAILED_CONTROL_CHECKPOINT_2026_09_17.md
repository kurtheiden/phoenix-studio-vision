# Prologue Key I -> J failed control preparation checkpoint — 2026-09-17

## Status and evidence boundaries

This documentation-only end-of-session checkpoint follows committed baseline `53917b6f65245f50b8834865e7c6e7d58e3575ff` (`Document Prologue SVP reference-field trace`). That checkpoint closes the bounded read-only reference-field branch. The preregistered Key I -> J hypotheses and earlier artifact observations remain as written; this document does not revise them.

Host file identities and repository state below were checked read-only. Studio Vision and guest Finder behavior, guest-file provenance, and crash-report details are **owner-reported observations**; guest HFS metadata and the preserved macOS crash report were not independently inspected for this checkpoint. No Phoenix run, guest interaction, project edit, or serialization analysis occurred.

## 1. Prepared host inputs and failed control

The preserved source is:

```text
/Users/kurtheiden/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master
```

The existing, **preserved** preparation directory is:

```text
/Users/kurtheiden/Documents/Phoenix Research/Prologue-Key-I-J-m2m75ogo
```

It contains `CONTROL INPUT/Prologue Master`, `EXPERIMENT INPUT/Prologue Master`, `CONTROL OUTPUT/`, `EXPERIMENT OUTPUT/`, and `PREPARATION.json`. The two inputs were independently copied directly from the preserved source using `/usr/bin/ditto --rsrc --extattr`. This host-side copying and its verification do not establish guest-native equivalence after transfer.

Host verification established that the source and both prepared inputs are regular files, each with a **33,057-byte**, byte-identical data fork and SHA-256:

```text
b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733
```

All three have exact matching host-visible FinderInfo (`MID2` / `MIDI`) and quarantine xattrs. No host-visible resource fork, readable named resource fork, or checked AppleDouble/`.rsrc` companion was found. Their paths and inodes differ; some access and metadata-change timestamps differ. The EXPERIMENT INPUT remained untouched. Both output directories were empty at the stopping point, and `PREPARATION.json` remained intact.

The owner transferred/copied the newly prepared **CONTROL INPUT** into the Mac OS 9 / SheepShaver workflow and opened that specimen in Studio Vision Pro 4.5. The project opened far enough to reach its UI, but project-window controls appeared nonresponsive and did not become responsive during a wait of **more than 15 minutes**. This does **not** establish an application-wide Studio Vision freeze or crash; prior observation showed Studio Vision could remain responsive enough to quit normally while project controls appeared nonresponsive. No intentional project edit and **no control save** occurred. No control output exists. The Key I -> J edit was **not performed**.

## 2. Guest Finder sequence and unresolved provenance

Before introducing the new control, a `Prologue Master` already existed on the native Mac OS 9 Desktop. The owner attempted to preserve it by dragging it into a new `Original Files` folder without intentionally holding Option to request a copy. A `Prologue Master` appeared in that folder while one remained on the Desktop. The owner dragged the Desktop file to `Original Files` again, accepted Finder's overwrite prompt, and still observed a `Prologue Master` on the Desktop. The owner moved that remaining Desktop file to Trash. **Trash was not emptied.** The newly prepared CONTROL INPUT was then copied into the guest workflow/Desktop and produced the project-control nonresponse above.

At the stopping point, the owner directly observed **three guest-side files named `Prologue Master`**:

| Guest location | Observed provenance and boundary |
|---|---|
| Desktop | Descendant of the newly prepared CONTROL INPUT; failed/nonresponsive control specimen. |
| `Original Files` | Derived from the older pre-existing Desktop specimen through the ambiguous drag/copy/overwrite sequence; exact guest-file-instance provenance unresolved. |
| Trash | Older Desktop instance remaining after that sequence, then moved to Trash; Trash not emptied. Its exact relationship to the `Original Files` instance is unresolved. |

**Do not identify either `Original Files` or Trash as the previously responsive guest-native instance without further evidence.** The observed Finder behavior is not established as a SheepShaver bug. Cross-volume/classic Finder copy behavior and other explanations remain possible. Matching host data forks and xattrs cannot establish exact guest HFS catalog/fork state; Studio Vision/SheepShaver runtime state also remains an unresolved possible contributor.

## 3. Subsequent SheepShaver crash

Later, while the owner was away, **SheepShaver itself crashed**. The owner preserved the macOS crash report and supplied these facts:

| Crash-report field | Owner-reported value |
|---|---|
| Process / version | SheepShaver 2.5 |
| Architecture / host OS | Native ARM64; macOS 15.7.5 |
| Launch | 2026-09-17 01:13:52 +0100 |
| Crash | 2026-09-17 01:30:23 +0100 |
| Exception / termination | `EXC_CRASH (SIGABRT)`; Abort trap 6 |
| Application-specific information | `abort() called` |
| Crashed thread | Main thread |
| Relevant stack | `powerpc_cpu::execute_illegal(unsigned int)` -> `powerpc_cpu::execute(unsigned int)` -> `main` -> `abort` |

The crash report establishes, as reported, that SheepShaver's PowerPC execution path reached `execute_illegal()` and SheepShaver aborted. It does **not** establish that Studio Vision or Prologue Master caused the crash, or that the crash and preceding project-control nonresponse share a cause. This checkpoint does not begin emulator debugging.

## 4. Experiment validity and interpretation

The planned causal edit remains **Sequences -> Frames 9-10 -> Key: I -> J**. It has not begun causally because no valid paired no-edit control save exists. The currently prepared host inputs were classified **unsuitable for proceeding** after the failed control, an operational judgment rather than a proven diagnosis of why the controls stayed nonresponsive. No experimental save is authorized from this failed pairing. The read-only reference-field branch remains **closed**. Do not alter the preregistered outcome classes in the previous preparation record.

Established at this stopping point: the prepared host data forks matched; the new control's project controls remained apparently nonresponsive for more than 15 minutes; no edit or save occurred; three same-named guest files were reported in distinct locations; and SheepShaver subsequently aborted. Unresolved: the exact provenance and guest-native metadata of the older two files, the cause of project-control nonresponse, and any causal relationship to the later SheepShaver crash. None of those unresolved questions may be silently converted into an explanation.

## 5. Exact next-session resume boundary

Begin with **inventory, not another experiment**. After SheepShaver is restarted in the next session, inspect the guest-native state **without moving files**: `Desktop/Prologue Master`, `Original Files/Prologue Master`, and `Trash/Prologue Master`. Preserve all three while deciding provenance/recovery strategy. Do not manufacture another host-side `Prologue Master` or introduce a fourth identically named guest file. Only after the three-file state is understood should a simpler controlled design with unique names be proposed. No restart, guest inventory, copy, edit, or save was performed for this checkpoint.

Preserve `/Users/kurtheiden/Documents/Phoenix Research/Prologue-Key-I-J-m2m75ogo` and its `PREPARATION.json` exactly as failed-attempt evidence. A revised experiment requires a new preparation directory/record or an explicitly versioned addendum; the current manifest and inputs must not be overwritten or repurposed.

## 6. Repository and production boundary

Before this file was created, HEAD, `main`, and `origin/main` matched `53917b6f65245f50b8834865e7c6e7d58e3575ff`, and staging was empty. The four protected unrelated local files were two modified unstaged documents (`docs/DECISIONS.md`, `docs/ROADMAP.md`) and two untracked documents (`docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md`, `docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md`). Their hashes were recorded before writing this checkpoint and must match afterward.

This checkpoint creates **only this new documentation file**. It changes no existing documentation, code, test, production parser, profile/hash/readiness gate, research artifact, or generated MIDI derivative. Nothing is staged, committed, or pushed; the new file remains untracked for review.
