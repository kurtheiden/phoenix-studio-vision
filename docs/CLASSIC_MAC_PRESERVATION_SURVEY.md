# Objective

This report documents the successful Finder round-trip test for Experiment 007
and inventories the StuffIt archive made from its known-good native Mac OS 9
baseline. The work is read-only evidence gathering. The archive was not
extracted, and no source artifact was modified.

# Provenance

The authentic `newest STUFF` project was opened in Studio Vision without an
intentional edit and saved as `newest STUFF baseline` on the native Mac OS 9
Desktop. That native save opened and functioned normally in Studio Vision. It
was then Finder-copied from the native Desktop to SheepShaver's `Unix` shared
volume, producing the host-visible representation inventoried below.

For the later round-trip test, the file on `Unix` was Finder-copied back to the
native Mac OS 9 Desktop. The user reports that the returned file opened and
functioned normally in Studio Vision and was not intentionally modified or
resaved during the test. The directly supported behavioral observation is that
this Studio Vision project, first saved on the native Mac OS 9 filesystem,
remained usable after this Finder-copy round trip through `Unix`.

After that test, the known-good native baseline was archived with StuffIt on
the native OS 9 Desktop. The archive was Finder-copied to `Unix` and then copied
from the host side into the Experiment 007 research directory.

The supplied directory name `Experiment 007 - Untouched Baseline Save` was not
present. Inspection found the closely matching directory
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline`,
containing exactly two files. Their actual names were used without renaming.

# Experiment 007 artifacts

| Artifact | Exact path | Basename | Filesystem/data-fork size | SHA-256 |
|---|---|---|---:|---|
| host-visible project | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 bytes | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| StuffIt archive | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline.sit` | `newest STUFF baseline.sit` | 89,525 bytes | `6a1a5135d308eeab896475550347bd8c22055dfb0f1130ffea28b60458e7d68c` |

The archive's 89,525-byte value is its host filesystem/data-fork size, not an
uncompressed member size.

# StuffIt archive observations

The host `file` utility identifies the artifact generically as `StuffIt
Archive`. Its data begins with the printable banner `StuffIt (c)1997-1998
Aladdin Systems, Inc.` followed by an Aladdin Systems StuffIt URL. Its Finder
Type is `SIT5` and Finder Creator is `SIT!`. These are directly observable
format/version indicators. They support identification as a StuffIt artifact
associated with the `SIT5` type, but the available observations do not
establish a more specific creating-application version.

Read-only raw inspection exposes the member name `newest STUFF baseline` once,
beginning at archive offset `0x000000a2`. The bytes `MID2MIDA` are also visible
in the adjacent header region, beginning at `0x000000bb`, consistent with
Finder Type `MID2` and Creator `MIDA` for that named member. Because the
installed `bsdtar` reports `Unrecognized archive format`, no installed archive
lister produced a parsed member listing. The visible name and Type/Creator are
therefore recorded as raw header observations, not as a complete parsed archive
directory.

The archive's host metadata is:

| Observation | Value |
|---|---|
| Finder Type/Creator | `SIT5` / `SIT!` |
| `com.apple.FinderInfo` | present, 32 bytes |
| `org.BasiliskII.FinderInfo` | present, 16 bytes |
| `org.BasiliskII.ExtendedFinderInfo` | present, 16 bytes |
| other extended attributes | none observed |
| resource fork | absent |

The raw `com.apple.FinderInfo` begins
`53 49 54 35 53 49 54 21 01 00`; the BasiliskII FinderInfo is
`53 49 54 35 53 49 54 21 01 00 ff ff ff ff 00 00`; and the extended
FinderInfo is 16 zero bytes.

Without a read-only parser capable of listing this archive, the following are
unknown: the parsed member count; whether separate data and resource forks are
explicitly represented; archived data-fork size; archived resource-fork size;
compression method; timestamps; Finder flags beyond the visible bytes; and any
other classic-Mac metadata. No fork preservation is inferred merely from
StuffIt's capabilities.

# Host-visible representation

The uncompressed host-visible project is 211,468 bytes with SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The generic host identification is `data`. Its Finder Type is `MID2` and its
Finder Creator is `MIDA`.

| Observation | Value |
|---|---|
| `com.apple.FinderInfo` | present, 32 bytes |
| `org.BasiliskII.FinderInfo` | present, 16 bytes |
| `org.BasiliskII.ExtendedFinderInfo` | present, 16 bytes |
| other extended attributes | none observed |
| resource fork | absent |

The complete `com.apple.FinderInfo` is the Type/Creator and flag bytes
`4d4944324d4944410100000000000000` followed by 16 zero bytes.
`org.BasiliskII.FinderInfo` is
`4d4944324d4944410100007b01080000`, and
`org.BasiliskII.ExtendedFinderInfo` is 16 zero bytes.

# Preservation evidence

The successful reopen after the native Desktop to `Unix` to native Desktop
Finder-copy round trip supports the usability of this specific Finder-copy
workflow for this specific Studio Vision artifact. It does not show that all
Studio Vision projects will survive the workflow, that direct Studio Vision
saves to `Unix` are always corrupt, or that the host-visible representation
preserves every classic-Mac filesystem property.

The StuffIt artifact is positively identified by its hash, generic file
identification, printable StuffIt banner, Finder Type/Creator, and visible
member-name and member-Type/Creator bytes. Its archive data fork has been
preserved as a host-visible 89,525-byte file at the recorded hash. The archive
itself has no host-visible resource fork.

The installed tools did not expose archived member bytes or a parsed archived
data-fork size. Consequently, byte identity between the archived member and
the 211,468-byte host-visible project has not been established. The evidence
also does not establish whether the archive contains a resource fork or every
classic-Mac filesystem property.

# Unknowns

- The archive's exact StuffIt format revision and creating-application version
  are unknown beyond the visible banner and `SIT5` Finder Type.
- A parsed member inventory is unavailable; only one raw member-name string was
  observed.
- Archived data- and resource-fork sizes and the presence of a separately
  represented resource fork are unknown.
- Byte identity between the archived project data fork and the host-visible
  Experiment 007 data fork is unknown.
- The native Desktop file's complete metadata before archiving and at each
  Finder-copy stage was not captured from the host side.
- The workflow's behavior for other Studio Vision artifacts is unknown.
- The archive has not been tested by restoring it, and this survey does not
  establish that it preserves every classic-Mac filesystem property.

# Recommended next step

Preserve the StuffIt archive unchanged and locate an already-available,
read-only StuffIt 5-capable listing tool or format specification. Use it first
to record the archive directory, fork sizes, Finder metadata, and integrity
status without extraction. Only a separately authorized restoration test on a
working copy should later compare restored fork bytes with the known-good
native baseline.
