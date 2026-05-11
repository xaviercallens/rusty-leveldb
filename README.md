# rusty-leveldb

[![crates.io](https://img.shields.io/crates/v/rusty-leveldb.svg)](https://crates.io/crates/rusty-leveldb)

> **Fork Notice**: This is a production fork of [leveldb-rs](https://github.com/dermesser/leveldb-rs) with performance optimizations. See [FORK_NOTICE.md](.github/FORK_NOTICE.md) for details.

A fully compatible implementation of LevelDB in Rust with performance optimizations including memory-mapped I/O, improved caching, and optimized bloom filters.

## Performance

Benchmark results (100K operations, Apple Silicon M-series):

| Operation | Ops/sec | Notes |
|-----------|---------|-------|
| Sequential Writes | ~808K | WAL + memtable |
| Sequential Reads (cold) | ~413K | mmap + block cache |
| Sequential Reads (warm) | ~412K | block cache hits |
| Random Reads | ~404K | bloom filter + mmap |
| Deletes | ~925K | Tombstone writes |

### Optimizations Applied

1. **Memory-mapped I/O** (`memmap2`): SSTable files >16KB are memory-mapped for zero-copy reads, eliminating syscall overhead on the hot read path.

2. **Enlarged block cache** (32 MB default): 4× larger than upstream, reducing repeated disk reads for working sets that fit in cache.

3. **Bloom filter bounds-check elimination**: `unsafe get_unchecked` in the bloom filter hot loop removes redundant bounds checks (the modulo arithmetic guarantees in-bounds access).

4. **Increased table cache** (2048 open files): Keeps more SSTable file handles cached, reducing open/close overhead.

### Running the Benchmark

```bash
cargo run --release --example bench
```

## Features

- Full LevelDB compatibility (reads/writes interoperable with C++ LevelDB)
- Async support via `tokio` or `async-std`
- Snappy compression
- Bloom filters for read optimization
- Memory-mapped I/O for SSTable reads
- LRU block cache with configurable capacity

## Usage

```rust
use rusty_leveldb::{DB, Options};

let mut opt = Options::default();
opt.create_if_missing = true;

let mut db = DB::open("my_database", opt).unwrap();

db.put(b"hello", b"world").unwrap();
assert_eq!(db.get(b"hello"), Some("world".as_bytes().into()));
db.delete(b"hello").unwrap();
db.flush().unwrap();
```

## Building

```bash
cargo build --release
cargo test
```

## License

MIT — see [LICENSE](LICENSE).

## Acknowledgments

- Original implementation: [dermesser/leveldb-rs](https://github.com/dermesser/leveldb-rs)
- Performance validation: [SocrateSuite](https://github.com/xaviercallens/socrateagora) SpectoRust pipeline
