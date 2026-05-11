# rusty-leveldb

[![crates.io](https://img.shields.io/crates/v/rusty-leveldb.svg)](https://crates.io/crates/rusty-leveldb)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A fully compatible, high-performance implementation of [Google LevelDB](https://github.com/google/leveldb) in Rust — with memory-mapped I/O, optimized bloom filters, and AI-generated C++ to Rust translations.

> **Fork of [dermesser/leveldb-rs](https://github.com/dermesser/leveldb-rs)** with performance optimizations and [SocrateSuite](https://github.com/xaviercallens/socrateagora) SpectoRust pipeline validation.

---

## Highlights

- **Full LevelDB compatibility** — reads and writes are interoperable with C++ LevelDB
- **Memory-mapped I/O** — zero-copy SSTable reads via `memmap2`
- **Optimized bloom filters** — bounds-check elimination in the hot path
- **Async support** — `tokio` and `async-std` backends
- **AI-assisted migration** — includes 36 functions translated from C++ LevelDB by the SocrateAssist SpectoRust pipeline (see [`spectorust-output/`](spectorust-output/))

---

## Performance

Benchmark on Apple Silicon (100K operations, `--release`):

| Operation | Ops/sec | Latency (per op) |
|-----------|---------|-----------------|
| Sequential Writes | **~642K** | 1.5 µs |
| Sequential Reads (cold) | **424K** | 2.3 µs |
| Sequential Reads (warm) | **425K** | 2.3 µs |
| Random Reads | **415K** | 2.4 µs |
| Deletes | **~386K** | 2.5 µs |

### Optimizations vs Upstream

| Optimization | Technique | Impact |
|---|---|---|
| Memory-mapped I/O | `memmap2` for SSTable files >16KB | ~2× read throughput |
| Block cache | 32 MB (4× upstream default) | ~15% repeated reads |
| Table cache | 2048 open files (2× upstream) | Reduced open/close overhead |
| Bloom filter | `unsafe get_unchecked` in hot loop | ~5-10% random reads |
| **Bloom filter fast-path** | Native endian `chunks_exact` loop | ~10% hashing speedup |
| **Block header parsing** | Single-byte varint coalescing | ~5% sequential scan |
| **Fixed32 native encoding**| Replaced trait traits with `from_le_bytes` | Eliminated `integer-encoding` bounds |

See [OPTIMIZATION_PLAN.md](OPTIMIZATION_PLAN.md) for the full roadmap.

### Run the Benchmark

```bash
cargo run --release --example bench
```

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rusty-leveldb = "4.0"
```

Basic usage:

```rust
use rusty_leveldb::{DB, Options};

fn main() {
    let mut opt = Options::default();
    opt.create_if_missing = true;

    let mut db = DB::open("my_database", opt).unwrap();

    // Write
    db.put(b"hello", b"world").unwrap();

    // Read
    let val = db.get(b"hello");
    assert_eq!(val, Some("world".as_bytes().into()));

    // Delete
    db.delete(b"hello").unwrap();
    db.flush().unwrap();
}
```

### Async Usage (Tokio)

```toml
[dependencies]
rusty-leveldb = { version = "4.0", features = ["async"] }
```

```rust
use rusty_leveldb::AsyncDB;

#[tokio::main]
async fn main() {
    let db = AsyncDB::new("async_db", Options::default()).unwrap();
    db.put(b"key".to_vec(), b"value".to_vec()).await.unwrap();
    let val = db.get(b"key".to_vec()).await.unwrap();
    assert_eq!(val, Some(b"value".to_vec()));
}
```

---

## SpectoRust: AI-Generated C++ to Rust Translations

The [`spectorust-output/`](spectorust-output/) directory contains **36 Rust functions** automatically translated from Google LevelDB's C++ source using the SocrateAssist hybrid 8-stage pipeline:

```
Bug Detection → Spec Generation → Translation → Optimization
→ Test Generation → Compilation Loop → Post-Processing → Validation
```

**Results**: 100% translation success rate, 0.90 average quality score.

These translations demonstrate automated C++ to Rust migration at production quality and serve as training data for reinforcement learning (RLCF).

See [`spectorust-output/README.md`](spectorust-output/README.md) for the full function list.

---

## Building

```bash
# Build
cargo build --release

# Run tests (146 tests)
cargo test

# Run benchmark
cargo run --release --example bench
```

### Features

| Feature | Description | Default |
|---------|-------------|---------|
| `fs` | Disk-based storage (POSIX) | ✅ |
| `async` | Tokio async wrapper | ❌ |
| `asyncdb-tokio` | Tokio backend | ❌ |
| `asyncdb-async-std` | async-std backend | ❌ |

---

## Architecture

```
┌─────────────────────────────────────────────┐
│              DB (db_impl.rs)                 │
│  put() / get() / delete() / flush()        │
├─────────────────────────────────────────────┤
│  MemTable (skipmap)  │  Immutable MemTable  │
├──────────────────────┴──────────────────────┤
│           Version Set (LSM tree)            │
│  L0: unsorted SSTables                      │
│  L1-L6: sorted, non-overlapping SSTables   │
├─────────────────────────────────────────────┤
│  Table Cache ──► Table Reader ──► Blocks    │
│       │              │                      │
│       ▼              ▼                      │
│  Block Cache    Bloom Filter                │
├─────────────────────────────────────────────┤
│  Env (disk_env.rs)                          │
│  ├── MmapRandomAccess (files >16KB)        │
│  └── File pread (small files)              │
└─────────────────────────────────────────────┘
```

---

## Project Structure

```
├── src/                    # Core library
│   ├── db_impl.rs         # Database implementation
│   ├── disk_env.rs        # Disk I/O with mmap
│   ├── table_reader.rs    # SSTable reader
│   ├── table_builder.rs   # SSTable writer
│   ├── cache.rs           # LRU block cache
│   ├── filter.rs          # Bloom filter
│   ├── memtable.rs        # In-memory table
│   ├── skipmap.rs         # Skip list
│   └── ...
├── examples/              # Example programs
│   ├── bench.rs           # Performance benchmark
│   └── ...
├── spectorust-output/     # AI-generated Rust from C++ LevelDB
│   ├── db/                # 15 database functions
│   ├── table/             # 6 SSTable functions
│   ├── util/              # 14 utility functions
│   └── hybrid_results.json
├── OPTIMIZATION_PLAN.md   # Performance roadmap
├── LICENSE                # MIT
└── Cargo.toml
```

---

## Contributing

Contributions welcome. Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Ensure all 146 tests pass (`cargo test`)
4. Run the benchmark to check for regressions (`cargo run --release --example bench`)
5. Open a pull request against `dev`

---

## License

MIT License — see [LICENSE](LICENSE).

**Original implementation**: [Lewin Bormann](https://github.com/dermesser) (2016)
**Performance optimizations & SpectoRust**: [Xavier Callens](https://github.com/xaviercallens) (2026)

---

## Related Projects

- [SocrateSuite](https://github.com/xaviercallens/socrateagora) — Autonomous software engineering platform
- [Google LevelDB](https://github.com/google/leveldb) — Original C++ implementation
- [dermesser/leveldb-rs](https://github.com/dermesser/leveldb-rs) — Upstream Rust implementation
