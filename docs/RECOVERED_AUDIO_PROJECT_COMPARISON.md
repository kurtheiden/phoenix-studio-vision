# Direct observations

This is a strictly observational, byte-level comparison of these two files:

- Original: `/Users/kurtheiden/Documents/Phoenix Research/Recovered Audio Projects/Project 001/original project/chris stuff`
- Repaired: `/Users/kurtheiden/Documents/Phoenix Research/Recovered Audio Projects/Project 001/repaired project/chris stuff with audio`

No project structure or resource was decoded. No meaning is assigned to any byte region, and this comparison does not attempt to determine how Studio Vision references audio.

The data forks differ in size and SHA-256 hash. Across the offset interval present in both files, most bytes differ. The repaired file also has an 84,772-byte tail beyond the end of the original file.

The resource forks differ: the original has no resource fork, while the repaired file has a 7,437-byte resource fork.

The files have the same three Finder-related extended-attribute names. Their values differ. The repaired file additionally has `com.apple.ResourceFork`.

# Measurements

## Methodology and classification rule

Measurements used `stat`, `shasum -a 256`, `cmp -l`, `xattr -px`, `xxd`, and `strings -a -t d`. Resource forks were read only through `..namedfork/rsrc`; they were not decoded.

Data-fork edit classes use a deliberately simple positional rule, not a format-aware alignment:

- At every absolute zero-based offset present in both files, unequal bytes are **changed** and equal bytes are unchanged.
- Bytes in the repaired file beyond the original file's EOF are **inserted**.
- Bytes in the original file beyond the repaired file's EOF would be **deleted**.

Consequently, “inserted” and “deleted” below describe length-relative positional observations. They are not claims about the operation that produced the repaired file. Alternative sequence alignments are possible and are listed under Unknowns.

## Data forks

| Measurement | Original | Repaired |
|---|---:|---:|
| Size | 176,823 bytes | 261,595 bytes |
| SHA-256 | `2d1ad318e1fbdd1d5b2265c90468e94f7c51916be361fc0c25b15b67e64f60b2` | `8a6a39ad25280ab8957419ffcdbf4311264455d8e0ffea485820230bf7bec40f` |

- Inserted range: repaired offsets **176,823–261,594**, length **84,772** bytes. This is one isolated, contiguous tail under the positional rule.
- Deleted ranges: **none** under the positional rule.
- Changed ranges: **8,556** maximal contiguous runs within offsets **1–176,822**, totaling **161,176** changed bytes.
- Unchanged bytes in the shared 176,823-byte interval: **15,647**.
- Smallest changed run: **1 byte**. Largest changed run: **1,455 bytes**.

Because listing 8,556 individual runs would obscure the observation, the runs are summarized below into measurement clusters. A cluster joins changed runs separated by at most 16 unchanged bytes. The columns are zero-based absolute start offset, inclusive end offset, span length, and number of exact changed runs. This produces 116 clusters: 96 contain one exact run and 20 contain multiple nearby runs.

```text
start   end      span     exact-runs
1       93       93       10
112     454      343      27
489     489      1        1
530     530      1        1
571     571      1        1
612     612      1        1
653     653      1        1
694     694      1        1
735     735      1        1
776     776      1        1
817     817      1        1
858     858      1        1
899     899      1        1
940     940      1        1
981     981      1        1
1022    1022     1        1
1063    1063     1        1
1104    1104     1        1
1145    1145     1        1
1186    1186     1        1
1227    1227     1        1
1268    1268     1        1
1309    1309     1        1
1350    1350     1        1
1391    1391     1        1
1432    1432     1        1
1473    1473     1        1
1514    1514     1        1
1555    1555     1        1
1596    1596     1        1
1637    1637     1        1
1678    1678     1        1
1719    1719     1        1
1760    1760     1        1
1801    1801     1        1
1842    1842     1        1
1883    1883     1        1
1924    1924     1        1
1965    1965     1        1
2006    2006     1        1
2047    2047     1        1
2088    2088     1        1
2129    2129     1        1
2170    2170     1        1
2211    2211     1        1
2252    2252     1        1
2293    2293     1        1
2334    2334     1        1
2375    2375     1        1
2416    2416     1        1
2457    2457     1        1
2498    2498     1        1
2539    2539     1        1
2580    2580     1        1
2621    2621     1        1
2662    2662     1        1
2703    2703     1        1
2744    2744     1        1
2785    2785     1        1
2826    2826     1        1
2867    2867     1        1
2908    2908     1        1
2949    2949     1        1
2990    2990     1        1
3031    3031     1        1
3072    3072     1        1
3113    3113     1        1
3154    3154     1        1
3195    3195     1        1
3236    3236     1        1
3277    3277     1        1
3318    3318     1        1
3359    3359     1        1
3400    3400     1        1
3441    3441     1        1
3482    3482     1        1
3523    3523     1        1
3564    3564     1        1
3605    3605     1        1
3646    3646     1        1
3687    3687     1        1
3728    3728     1        1
3769    3769     1        1
3810    3810     1        1
3851    3851     1        1
3892    3892     1        1
3933    3933     1        1
3958    5557     1600     105
5575    5581     7        1
5611    5743     133      7
5761    5769     9        1
5797    5876     80       6
5895    5901     7        1
5919    5929     11       1
5947    5954     8        1
5983    6115     133      6
6133    6140     8        1
6169    6301     133      6
6319    6325     7        1
6355    6487     133      6
6505    6511     7        1
6541    6706     166      28
6727    6806     80       4
6825    6831     7        1
6849    6882     34       3
6913    7020     108      12
7039    7045     7        1
7063    7075     13       1
7103    7235     133      19
7289    7358     70       4
7407    7449     43       2
7471    25621    18151    1873
25640   25809    170      27
25840   26649    810      137
26680   69983    43304    1579
70011   176822   106812   4599
```

At a 16-byte separation threshold the changes are therefore both isolated and clustered: the regularly spaced one-byte changes from offsets 489 through 3,933 are isolated clusters, while later changes form large, dense clusters. At a 64-byte threshold all changed runs join into one cluster, so “clustered” is threshold-dependent.

## Printable strings

`strings -a -t d` (minimum length 4, decimal offsets) measured 8,453 printable runs in the original and 8,472 in the repaired file. Comparing distinct string values gives 298 values found only in the original output and 338 found only in the repaired output. This method can emit accidental printable runs from arbitrary binary bytes; the strings below are observations only.

Selected exact differences, with zero-based decimal offsets:

| Original occurrence | Repaired occurrence |
|---|---|
| offset 14: `JD-800` | offset 14: `QuickTime Music,F` |
| offset 59: `JD-990 w/Vintage` | offset 59: `JD-800ime Music,F` |
| offset 104: `Juno-106/Vintage` | no identical repaired-only counterpart identified at that offset |
| offset 149: `JUNO-606/Vintage` | no identical repaired-only counterpart identified at that offset |
| offset 194: `JV-10806/Vintage` | no identical repaired-only counterpart identified at that offset |
| offset 284: `JV-880 Vintagege` | offset 284: `JD-990 w/VintageF` |
| offset 329: `QuickTime Musice` | offset 329: `JUNO-60w/VintageF` |
| offset 11,385: `COMPRHND.MID` | offset 31,403: `COMPRHND.MID` |
| no original occurrence | offset 43,421: `Comprehend take 1.norm` |
| no original occurrence | offset 81,835: `alone take 1.norm` |
| no original occurrence | offset 193,971: `Hardworld take 1.norm` |
| no original occurrence | offset 237,775: `Walk around take 1.norm` |

The full `strings` output also contains hundreds of short punctuation/alphanumeric fragments. Those fragments were counted above but are not interpreted as text fields.

## Resource forks

| Measurement | Original | Repaired |
|---|---:|---:|
| Resource fork present | No | Yes |
| Size | 0 bytes / absent | 7,437 bytes |
| SHA-256 | not applicable | `b11c6f2170b71094ebc4dc663083f06f89825a9eedb5f35f70af0970e88a1559` |

The repaired resource-fork hash agrees whether read through `com.apple.ResourceFork` or `..namedfork/rsrc`. No resource data was decoded.

## Extended attributes

| Attribute | Original size and SHA-256 | Repaired size and SHA-256 | Result |
|---|---|---|---|
| `com.apple.FinderInfo` | 32; `ff4987ee390167baabb168c33f905531303df588749782befaa41933e54adc0a` | 32; `c26aa49e41ab034b0dc30e553a8ae014df446b72843a33ab3610d47fc348aeba` | differs |
| `org.BasiliskII.ExtendedFinderInfo` | 16; `518be89d411f8fcd1084f2160e0fb3c4b9a09530a29a3d639459e2ba36e3d862` | 16; `374708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb` | differs |
| `org.BasiliskII.FinderInfo` | 16; `3cbdca67e1fc46667184312e40f7bc2a702781c2b799a9b977fe77e77a16ce20` | 16; `28400c20c93b4c4d0fa55aac7d90a6a2f68015ea21e9c5cb6c947a5e80134761` | differs |
| `com.apple.ResourceFork` | absent | 7,437; `b11c6f2170b71094ebc4dc663083f06f89825a9eedb5f35f70af0970e88a1559` | repaired only |

## Finder metadata

Finder metadata is reported separately even though it is physically stored in extended attributes.

| Value | Original hexadecimal bytes | Repaired hexadecimal bytes |
|---|---|---|
| `com.apple.FinderInfo` | `4D4944534D49444101000000000000000000000000000000FDBD000000000000` | `4D4944534D494441010000000000000000000000000000000000000000000000` |
| `org.BasiliskII.FinderInfo` | `4D4944534D4944410100FFFFFFFF0000` | `4D4944534D494441010002C001010000` |
| `org.BasiliskII.ExtendedFinderInfo` | `0000000000000000FDBD000000000000` | `00000000000000000000000000000000` |

The first eight bytes of both FinderInfo values are identical (`4D 49 44 53 4D 49 44 41`); other bytes differ as shown. These values are not decoded into type, creator, flags, coordinates, or other Finder fields.

Filesystem observations: both files are regular files with mode `-rw-r--r--`, owner `kurtheiden`, group `staff`, and no `stat` file flags. Their filenames, inode numbers, sizes, and timestamps differ.

# Unknowns

- Whether a different byte-sequence alignment would describe some positional “changed” bytes as interior insertions or deletions.
- Whether any matching byte subsequence is structurally significant.
- Whether any printable run is an intentional text value rather than coincidental bytes.
- What any data-fork, resource-fork, or FinderInfo region represents.
- Which process created or altered either file and in what order.
- Whether timestamps were preserved from an earlier filesystem or generated during recovery/copying.

# Hypotheses requiring further evidence

- The repaired file may have been produced by an application operation that rewrote much of the shared offset range and extended the data fork. Establishing this would require controlled samples or provenance records.
- Some repaired-only printable runs may correspond to external filenames. Establishing that they are fields, references, or active values would require format evidence; this comparison does not make that claim.
- The repaired-only resource fork may have been added or restored during repair. Establishing its origin or purpose would require comparison with known-good files or resource-level investigation, which was deliberately not performed here.
- The differing Finder metadata may reflect a copy, emulator, recovery, or application save operation. The byte differences alone do not select among those possibilities.

# Recommended next investigations

1. Preserve cryptographic hashes and acquire read-only forensic copies, including forks and extended attributes, before further handling.
2. Compare both files with multiple independently known-good Studio Vision projects of the same provenance using the same fork-separated measurements.
3. Generate controlled before/after projects in Studio Vision, changing one user-visible item at a time, then compare them without assuming field meanings.
4. Obtain filesystem or recovery-tool logs that could establish when and how the repaired file and its resource fork were created.
5. If authorized as a separate investigation, retain a complete machine-readable list of all 8,556 exact changed runs and both complete `strings` outputs; avoid treating heuristic alignment as ground truth.
