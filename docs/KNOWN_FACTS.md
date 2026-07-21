# Known Facts

This document records direct observations from local samples. Observations do
not establish a file signature, structure, or parser behavior.

## `newest-stuff-001`

- The local sample identifier is `newest-stuff-001`.
- The file has no extension.
- The file size is 171953 bytes.
- Its SHA-256 digest is
  `c44d415a4b69d56abd5680652ed99039a4f9ca9afd281898601ccc14026aebec`.
- Finder metadata identified the type code `MID2` and creator code `MIDA`.
- Printable device-related strings occur within the first 256 bytes, including:
  - `IAC Bus #1`
  - `JD-800s #1`
  - `JD-990s #1`
  - `Juno-106#1`
  - `JV-1080`
  - `JV-880`
- Repeated byte patterns surround several device names.
- No interpretation of those structures is confirmed yet.

## Artifact inventory observations

- Finder creator code `MIDA` was observed on `newest-stuff-001`, the
  `StudioVision.app` application, and the `Studio Vision Setup` artifact.
- Finder type code `APPL` was observed on the OMS 2.3.8 installer and the
  `StudioVision.app` application.
- Finder type code `MIDS` was observed on `Studio Vision Setup`; type code
  `MID2` was observed on `newest-stuff-001`.
- Resource forks were directly present as extended attributes on the OMS 2.3.8
  installer, `StudioVision.app`, and `Studio Vision Setup`.
- The OMS 2.3.8 installer resource fork is 295885 bytes, the
  `StudioVision.app` resource fork is 2112471 bytes, and the
  `Studio Vision Setup` resource fork is 286 bytes.
- The `StudioVision.app` data fork was identified by `file` as a PowerPC PEF
  executable. Its data fork begins with `Joy!peffpwpc`, and its `cfrg` resource
  contains `pwpc` and `StudioVision PPC`.
- The Studio Vision Pro 4.5 `Documentation` directory inventory contains five
  PDF 1.2 documents and one non-document `Icon\r` entry. Sizes, SHA-256
  digests, filesystem dates, and directly observable PDF metadata were
  recorded for those entries.

These observations do not establish general Finder-code meanings, resource
fork semantics, file-format structures, parser behavior, or compatibility.
