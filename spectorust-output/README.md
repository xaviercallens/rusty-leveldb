# SpectoRust Output — LevelDB C++ to Rust Translation

This directory contains **36 Rust functions** automatically translated from Google's [LevelDB](https://github.com/google/leveldb) C++ source code using the SocrateAssist hybrid 8-stage pipeline.

## Pipeline Details

- **Source**: Google LevelDB (20K LOC C++)
- **Method**: Hybrid SpectoRust pipeline (open-weight + premium LLM)
- **Stages**: Bug detection → Spec generation → Translation → Optimization → Test generation → Compilation loop → Post-processing → Validation
- **Success rate**: 100% (36/36 functions translated)
- **Average quality score**: 0.90/1.0
- **Duration**: 36 minutes

## Directory Structure

```
spectorust-output/
├── db/           — Database layer functions (15 files)
├── helpers/      — Environment helpers (1 file)
├── table/        — SSTable operations (6 files)
├── util/         — Utility functions (14 files)
├── hybrid_results.json — Full pipeline results with metrics
└── README.md     — This file
```

## Generated Functions

### db/
| File | Original C++ | Description |
|------|-------------|-------------|
| BuildTable.rs | db/builder.cc | Builds SSTable from iterator |
| ClipToRange.rs | db/db_impl.cc | Clamps value to range |
| DeleteEntry.rs | db/db_impl.cc | LRU cache entry deletion |
| DumpInternalIter.rs | db/db_impl.cc | Debug dump of internal iterator |
| GetInternalKey.rs | db/dbformat.cc | Extract internal key |
| GetLengthPrefixedSlice.rs | db/dbformat.cc | Decode length-prefixed slice |
| GuessType.rs | db/dumpfile.cc | Guess file type from name |
| HandleDumpCommand.rs | db/dumpfile.cc | Handle dump CLI command |
| InitTypeCrc.rs | db/log_writer.cc | Initialize CRC table |
| InternalKeyEncodingLength.rs | db/dbformat.cc | Key encoding length |
| MakeFileName.rs | db/filename.cc | Generate DB file paths |
| PackSequenceAndType.rs | db/dbformat.cc | Pack seq + type into u64 |
| RepairDB.rs | db/repair.cc | Database repair entry point |
| SaveError.rs | db/builder.cc | Save error status |
| TargetFileSize.rs | db/version_set.cc | Compute target file size |

### table/
| File | Original C++ | Description |
|------|-------------|-------------|
| DecodeEntry.rs | table/block.cc | Decode block entry |
| DeleteBlock.rs | table/table.cc | Delete block handle |
| NewEmptyIterator.rs | table/iterator.cc | Create empty iterator |
| NewMergingIterator.rs | table/merger.cc | Create merging iterator |
| NewTwoLevelIterator.rs | table/two_level_iterator.cc | Two-level iterator |
| ReadBlock.rs | table/format.cc | Read block from file |

### util/
| File | Original C++ | Description |
|------|-------------|-------------|
| AppendNumberTo.rs | util/logging.cc | Append number to string |
| BloomHash.rs | util/bloom.cc | Bloom filter hash |
| BytewiseComparator.rs | util/comparator.cc | Byte-wise key comparator |
| EncodeFixed32.rs | util/coding.cc | Fixed-width u32 encoding |
| GetWindowsErrorMessage.rs | util/env_windows.cc | Windows error messages |
| Hash.rs | util/hash.cc | Hash function |
| Log.rs | util/logging.cc | Logging utilities |
| NewLRUCache.rs | util/cache.cc | LRU cache constructor |
| PosixError.rs | util/env_posix.cc | POSIX error handling |
| PutFixed32.rs | util/coding.cc | Write fixed u32 |
| RandomSeed.rs | util/random.h | Random seed generation |
| RandomString.rs | util/testutil.cc | Random string generation |
| ReadUint32LE.rs | util/coding.cc | Read little-endian u32 |
| Value.rs | util/cache.cc | Cache value wrapper |

## Usage

These translations serve as:
1. **Training data** for RLCF (Reinforcement Learning from Codex Feedback)
2. **Validation** of the SpectoRust pipeline on real-world C++ code
3. **Reference implementations** for comparing idiomatic Rust patterns

## License

The original LevelDB source is BSD-licensed. These translations are provided under the same terms.
