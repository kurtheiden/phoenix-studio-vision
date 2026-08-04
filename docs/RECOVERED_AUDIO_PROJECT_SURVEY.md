# Recovered Audio Project Survey

This document records a read-only forensic survey of the external `Recovered
Audio Projects` collection on 2026-08-04. The surveyed material contains one
project directory, two Studio Vision project files, 49 associated audio files,
and three `.DS_Store` files. The `.DS_Store` files are not project or audio
samples and are excluded from the registries below. No source file was
modified.

This survey catalogs each file independently. It does not parse either project,
compare their binary contents, determine how Studio Vision represents audio,
or establish that a project string refers to any associated file.

## Method

- File sizes describe complete data forks in bytes.
- SHA-256 digests cover complete data forks only. Resource-fork attributes are
  reported separately.
- Finder Type and Creator are the first eight bytes of the observed
  `com.apple.FinderInfo` extended attribute.
- Extended attributes were enumerated without modification. Resource-fork
  presence means that a `com.apple.ResourceFork` attribute exists on the
  surveyed copy; absence does not establish historical absence.
- Printable strings are maximal runs of bytes from `0x20` through `0x7e`, with
  a minimum length of four bytes. Counts include coincidentally printable
  binary runs. Displayed strings are literal observations selected for their
  obvious audio-related vocabulary; their presence does not establish a field,
  record, path, or external-file reference.
- An audio file is identified as AIFF only when its data fork directly begins
  with an IFF `FORM` header whose form type is `AIFF`. Finder metadata is
  reported separately and is not treated as a data-fork signature.
- The two Studio Vision project files were inspected independently. No
  original-versus-repaired byte or string comparison was performed.

## Direct observations

### Studio Vision project files

#### `original project/chris stuff`

- Filename: `chris stuff`.
- Size: 176,823 bytes.
- SHA-256: `2d1ad318e1fbdd1d5b2265c90468e94f7c51916be361fc0c25b15b67e64f60b2`.
- Finder metadata: Type `MIDS`, Creator `MIDA`.
- Extended attributes: `com.apple.FinderInfo`,
  `org.BasiliskII.ExtendedFinderInfo`, and `org.BasiliskII.FinderInfo`.
- No `com.apple.ResourceFork` attribute was observed.
- Printable ASCII scan: 8,453 runs of at least four bytes.
- Obvious audio-related literal observations include `Audio-1` through
  `Audio-16`, `Audio/Video`, `44100`, `S600%`, `take 1`, `take 2`, `backing
  vocal`, `chris stuff Audio 1`, `Comprehend take 1.norm`, `comprehend take
  2.norm`, `comprehend temp music`, `alone take 1.norm`, `alone take 2.norm`,
  `alone harmony 1.norm`, `Hardworld take 1.norm`, `hardworld take 2.norm`,
  `hardworld take 3.norm`, `hardworld MUSIC`, `Walk around take 1.norm`,
  `walk around take 2.norm`, `Walk around take 3.norm`, `walk around take
  4.norm`, `walk around backup 1.norm`, `walk around delay 1.norm`, and `walk
  around mix 2`.
- Additional later printable runs include case- and punctuation-varying forms
  of the preceding observations, plus `comprehend backup 1 norm`, `walkaround
  take 1 norm`, `walk around backup p/down 1`, `Walk Around backup p/up 1`,
  `walk around take 1 partial norm`, `walk around take 1 p/down 1.1`, `walk
  around delay 2`, and `walk around delay 3`.

#### `repaired project/chris stuff with audio`

- Filename: `chris stuff with audio`.
- Size: 261,595 bytes.
- SHA-256: `8a6a39ad25280ab8957419ffcdbf4311264455d8e0ffea485820230bf7bec40f`.
- Finder metadata: Type `MIDS`, Creator `MIDA`.
- Extended attributes: `com.apple.FinderInfo`, `com.apple.ResourceFork`,
  `org.BasiliskII.ExtendedFinderInfo`, and `org.BasiliskII.FinderInfo`.
- The `com.apple.ResourceFork` attribute contains 7,437 bytes and has SHA-256
  `b11c6f2170b71094ebc4dc663083f06f89825a9eedb5f35f70af0970e88a1559`.
- Printable ASCII scan: 8,472 runs of at least four bytes.
- Obvious audio-related literal observations include `Audio-1` through
  `Audio-16`, `Audio/Video`, `44100`, `S600%`, `take 1`, `take 2`, `backing
  vocal`, `chris stuff Audio 1`, `Comprehend take 1.norm`, `comprehend take
  2.norm`, `comprehend temp music`, `alone take 1.norm`, `alone take 2.norm`,
  `alone harmony 1.norm`, `Hardworld take 1.norm`, `hardworld take 2.norm`,
  `hardworld take 3.norm`, `hardworld MUSIC`, `Walk around take 1.norm`,
  `walk around take 2.norm`, `Walk around take 3.norm`, `walk around take
  4.norm`, `walk around backup 1.norm`, `walk around delay 1.norm`, and `walk
  around mix 2`.
- Additional later printable runs include case- and punctuation-varying forms
  of the preceding observations, plus `comprehend backup 1 norm`, `walkaround
  take 1 norm`, `walk around backup p/down 1`, `Walk Around backup p/up 1`,
  `walk around take 1 partial norm`, `walk around take 1 p/down 1.1`, `walk
  around delay 2`, `walk around delay 3`, `comprehend temp music L.aiff`, and
  `hardworld MUSIC.L`.

### Associated audio files

The `audio files` directory contains 49 non-hidden files. Forty-five data forks
have directly observed `FORM`/`AIFF` headers. Four data forks have no container
identified by this survey; those four carry Finder Type `Sd2f` and Creator
`Sd2a`. This is a metadata observation, not a claim that their data-fork layout
has been validated.

All 49 files have `com.apple.FinderInfo`,
`org.BasiliskII.ExtendedFinderInfo`, and `org.BasiliskII.FinderInfo`. A
`com.apple.ResourceFork` attribute is present on 45 files,
`com.apple.quarantine` on six, and `com.apple.lastuseddate#PS` on two. These
attribute counts describe only the surveyed copies.

| Filename | Size (bytes) | SHA-256 | Direct format observation |
| --- | ---: | --- | --- |
| `alone harmony 1 norm` | 14569364 | `091826e15bd9b2aecbb56bc9eb431d9066c703155e7f18dadc2f5c30a8948c83` | AIFF (`FORM`/`AIFF`) |
| `alone take 1 norm` | 19975974 | `681c32ea70ff2c2208fd8ef895d90dac04a86b878af83e143b072016faa53af9` | AIFF (`FORM`/`AIFF`) |
| `alone take 2 norm` | 22411698 | `e28de98e1a50ce8266bc68862cdeaf55adec31dc5b6f8947972c17453394f4eb` | AIFF (`FORM`/`AIFF`) |
| `analog (saw wave chorused)` | 776580 | `9aeada3a630409139750a1c1e6c609d0788df13eabe7a251e373f1c2a7383d14` | AIFF (`FORM`/`AIFF`) |
| `analog (saw wave)` | 632834 | `5dbfe91d2c7bb2130175f384c787105efeaf756218da2ced95c2fc3be1d632cb` | AIFF (`FORM`/`AIFF`) |
| `analog (sawwave norm)` | 632834 | `962f890310da1d29261df6acbe46cc51270e71e4ee54ca6174031f0b1fe56b5d` | AIFF (`FORM`/`AIFF`) |
| `analog harmony 1 norm` | 3262666 | `6776648507b5b9416c1b663ae0086c318231f512156a8a347ead1d2612f0f4ae` | AIFF (`FORM`/`AIFF`) |
| `analog take 1 norm` | 11059424 | `690699e6f660fa7ef9a3e49727b956180b1952170823e3db0c1b3e7e12be8e1c` | AIFF (`FORM`/`AIFF`) |
| `analog take 2.5 norm` | 6013642 | `31869e4202a5b17a117a421d017ddff2d44e765bb53d5ebd8f2c0baab82d076f` | AIFF (`FORM`/`AIFF`) |
| `analog take 3 norm` | 9403786 | `a8090829278a60bf34752e52ab76d9596c506f4b2a56f78cd1f39a1e94cb2386` | AIFF (`FORM`/`AIFF`) |
| `compehrend take 2 norm` | 11191378 | `b388956eafb563f1078aa7f67e6e6466bfe68c5ac403df7ce4b85bee84991658` | AIFF (`FORM`/`AIFF`) |
| `comprehend backup 1 norm` | 11250192 | `29373d3937d7fb68e95e51a92ef98a218be927c9d8f3986b093034564b48a49b` | AIFF (`FORM`/`AIFF`) |
| `Comprehend take 1` | 11744758 | `07335bed6621e7e4df161877dca58bc124029add86f45cb644496b63c3330ff2` | AIFF (`FORM`/`AIFF`) |
| `comprehend take 1 norm` | 11744400 | `987429f75af6e9623ae3f9d3bbd3e3be1d2b55a2d8b87932b8cb6738c1c9cf4d` | AIFF (`FORM`/`AIFF`) |
| `comprehend take 2` | 11191378 | `f13821f24842f048d3ce184db6876b274e2bf7abd76bb9c0cbf3822e9230a892` | AIFF (`FORM`/`AIFF`) |
| `comprehend temp music L.aiff` | 5545470 | `c583cea501a1baf2f645d75bde9366dfd087d015e6a4c544aec519c23d18fee6` | AIFF (`FORM`/`AIFF`) |
| `comprehend temp music R.aiff` | 5545470 | `412b0d680c333a7d3a4475ba49df94e9b776f9858b9bd6d539c6b1eff04726e8` | AIFF (`FORM`/`AIFF`) |
| `comprehend temp music.L` | 11090832 | `abc7ceae529798cfd18e764c532458fc88eb890a984782c6ca4d676fbb4ea730` | No data-fork container identified; Finder `Sd2f` / `Sd2a` |
| `comprehend temp music.R` | 11090832 | `c1fd3a47b525543f0ad25620e47b6c8224733f64c689a2d8226cae77fd3bb617` | No data-fork container identified; Finder `Sd2f` / `Sd2a` |
| `hardworld MUSIC.L` | 23317128 | `b595af52a882e82e611d55eba387634281aadf0fa5843f24f74a0fe3e620829e` | No data-fork container identified; Finder `Sd2f` / `Sd2a` |
| `hardworld MUSIC.R` | 23317128 | `378b3b62e1049e03c8178cea8a4449faaff45e22ca584a327d4ad7a2c521d4be` | No data-fork container identified; Finder `Sd2f` / `Sd2a` |
| `Hardworld take 1` | 22380978 | `912abd3c49b2a7606411e8b51120379afac1c853f07272bce8d77f91e6f9f7d9` | AIFF (`FORM`/`AIFF`) |
| `hardworld take 1 norm` | 22380978 | `04396dde759add50a2d6736e33d6730a5ae515aeb0b585a41a09c8f588965597` | AIFF (`FORM`/`AIFF`) |
| `hardworld take 2` | 22186858 | `9947d600c9450a0b28928fedaf00c1818d3698db095a89764ef6ab7e9e64b67b` | AIFF (`FORM`/`AIFF`) |
| `hardworld take 2 norm` | 22186858 | `c47ab8a6434d880ce27b997a2be3f11d73d4db2d8b43eaea366b41d9b03a02d3` | AIFF (`FORM`/`AIFF`) |
| `hardworld take 3` | 20264790 | `a8f4266e11b07ca3aed593282da7a4ce610b85a3500808604cb6ba4c065b5aff` | AIFF (`FORM`/`AIFF`) |
| `hardworld take 3 norm` | 20264790 | `3a74c1da3b11e24937ee2118a6fda6eb96e7e86989c43c3a2c3a238bc08a109f` | AIFF (`FORM`/`AIFF`) |
| `Sweeper take 1 norm` | 13252206 | `3202cb5b25c4cb16a95d464124fdea7fe42ed7d202aea61580d8329635062987` | AIFF (`FORM`/`AIFF`) |
| `Sweeper2 music` | 34329692 | `9e4932d7eb2e56887178c77344da08d9f74eae82bd666f9b371228f66d8a9c4e` | AIFF (`FORM`/`AIFF`) |
| `walk around backup 1` | 16840156 | `95386fc3e0e04bb702c4abfdfccbf74bc84b574fcad067de04a1a052d73d6848` | AIFF (`FORM`/`AIFF`) |
| `walk around backup 1 norm` | 16814890 | `897e5c9eec3c2dbaa462f4c71eed3885a3d3aed4664570a11a8d6d31109df0d3` | AIFF (`FORM`/`AIFF`) |
| `walk around backup p:down 1` | 160770 | `12c2139fbbb2ef62165d8a1d85bae12b7cca37ef9d910d1c9f37fa0f083dcabf` | AIFF (`FORM`/`AIFF`) |
| `Walk Around backup p:up 1` | 402030 | `1b623cda211f9b2bae77d98953baa0c6d55c3cac319b69f89a7c2ad88d5fff48` | AIFF (`FORM`/`AIFF`) |
| `walk around delay 1` | 2760312 | `eaf8a03930de863564a4f1b08a1161944495c6622dd075791d8a99ac2210c19b` | AIFF (`FORM`/`AIFF`) |
| `walk around delay 1 norm` | 924542 | `266e63b04854e433d1050c56dc5f27df97eda9f5a952ec3412eceea1d5ed591d` | AIFF (`FORM`/`AIFF`) |
| `walk around delay 2` | 1224938 | `03974361ebdf68950b8036aed23d39b783391269819448102435cf95c326acc6` | AIFF (`FORM`/`AIFF`) |
| `walk around delay 3` | 611856 | `a84d78633d621085d621fdde9fbadf0b67c76dd1289715b1acb86ee3f98dc992` | AIFF (`FORM`/`AIFF`) |
| `walk around mix 2` | 33921972 | `99bb98f77589eb8d45e622ac8e8f12927c70cae8c7e4f266d162a1b838900367` | AIFF (`FORM`/`AIFF`) |
| `Walk around take 1` | 16863498 | `861db23194d8225440371d13bb19f17323f462339804102dafface6103e47c88` | AIFF (`FORM`/`AIFF`) |
| `walk around take 1 p:down 1` | 258938 | `2faf68be0d944e5d6378ae81227f7327a067bd5c8eda920aa9209f2be6b746f4` | AIFF (`FORM`/`AIFF`) |
| `walk around take 1 p:down 1.1` | 257424 | `5c1cca0809b631ea4bbfdbe1d563682ec64798860aaf0b4cbb90e6b7e2ff5204` | AIFF (`FORM`/`AIFF`) |
| `walk around take 1 partial norm` | 446464 | `ba03a0ccb03f6b0674d20464be0b80205ef1b6165677d9f96267696d709c59f0` | AIFF (`FORM`/`AIFF`) |
| `walk around take 2` | 5917038 | `b5aaeff6c955e33bcfa4ddfcd771915483f4de2b7dc33d519ecaafe68dbd288b` | AIFF (`FORM`/`AIFF`) |
| `walk around take 2 norm` | 5914108 | `fc4a70dfa4e608dcdcb26ab5edad9d97d45b9ddc8b35ff4521ab09de7d89ccfd` | AIFF (`FORM`/`AIFF`) |
| `Walk around take 3` | 16861460 | `f0aee200d67f9a966c837779b686a00d796ecb1f329d6675d4e20b195e5f0e4d` | AIFF (`FORM`/`AIFF`) |
| `walk around take 3 norm` | 16927024 | `bd82b9d586416868605df910a41d26007adba1ab510c3775af63daaaf5b474e3` | AIFF (`FORM`/`AIFF`) |
| `walk around take 4` | 16838044 | `9916205a313e98d5f12c0dfc63f3aa199e0c7eb24db3669fd9de989edcf9c74c` | AIFF (`FORM`/`AIFF`) |
| `walk around take 4 norm` | 16865474 | `0f246661a763cf60670b864cf34c49167aec0d2685b6c58d1c4d22503489266d` | AIFF (`FORM`/`AIFF`) |
| `walkaround take 1 norm` | 16859016 | `c4b7f169567a3cd79950bb1b2e1270ccfa2fb4725c093dfb2307867f1ba181ee` | AIFF (`FORM`/`AIFF`) |

## Unknowns

- The structures, boundaries, encodings, and purposes of the observed project
  bytes and printable strings remain unknown.
- This survey does not establish whether any project string is a filename,
  pathname, label, cached value, or external-file reference.
- The relationship, if any, between either project file and any associated
  audio file is unknown.
- The data-fork structure of the four `Sd2f` / `Sd2a` files was not identified.
  No conclusion is drawn from generic content detection on their apparent raw
  sample bytes.
- The contents and roles of resource forks were not parsed. Attribute presence
  on recovered copies does not establish what metadata existed on original
  media or during earlier transfers.
- The significance of spelling, case, punctuation, repeated strings, and
  visible sample-rate-like text is unknown.

## Hypotheses

No structural or reference hypothesis is advanced by this survey. The
audio-related vocabulary is sufficient to motivate controlled investigation,
but not to support a claim about how Studio Vision stores or resolves audio.

## Future investigation

- Preserve these filenames, sizes, hashes, Finder metadata, extended-attribute
  inventories, and resource forks as the baseline for later work.
- When explicitly authorized, compare the two project data forks and resource
  forks separately, reporting byte observations before proposing structure.
- Examine repeated audio-related strings in bounded context without assuming
  that literal similarity establishes a reference.
- Research the directly observed `Sd2f` / `Sd2a` metadata and inspect those
  files' resource-fork structures independently before assigning a format.
- If reference behavior is later tested, use controlled copies and
  one-variable experiments; do not modify this recovered collection.
