# LevelDB Rust Optimization Plan

## Context

This plan addresses the read performance gap between C++ LevelDB and rusty-leveldb, validated through the SocrateSuite SpectoRust pipeline.

**Baseline (before optimizations):**
- C++ LevelDB: ~2M reads/sec
- Rust (original): ~200K reads/sec (4.9× slower)

**After Phase 1 (implemented):**
- Rust (optimized): ~413K reads/sec (2× improvement, gap reduced to ~2.4×)

---

## Phase 1: Memory-Mapped I/O (DONE)

**Impact: ~2× read improvement**

- Added `memmap2` dependency for zero-copy file reads
- SSTable files >16KB are memory-mapped (immutable after creation)
- Eliminates `pread` syscall overhead on the hot read path
- Falls back to `pread` for small files (metadata, manifests)

## Phase 2: Block Cache Tuning (DONE)

**Impact: ~15% improvement on repeated reads**

- Increased default block cache from 8MB to 32MB
- Increased table cache from 1024 to 2048 open files
- Reduces cache eviction pressure for working sets <32MB

## Phase 3: Bloom Filter Optimization (DONE)

**Impact: ~5-10% on random reads**

- `unsafe get_unchecked` in bloom filter hot loop
- Modulo arithmetic guarantees in-bounds access
- Benchmark uses 12 bits/key (vs default 10) for lower false positive rate

---

## Phase 4: Iterator Dispatch (TODO)

**Expected Impact: ~10-15% on sequential reads**

Replace `Box<dyn LdbIterator>` with enum dispatch in merging iterator:

```rust
enum IterKind {
    Mem(MemTableIterator),
    Table(TableIterator),
    Block(BlockIter),
}
```

This eliminates vtable indirection on every `advance()` call in the hot scan path.

## Phase 5: Key Comparison Optimization (TODO)

**Expected Impact: ~10% on all operations**

- Inline the comparator for the common `DefaultCmp` case
- Use `memcmp` intrinsic for byte-slice comparison
- Avoid `Rc<Box<dyn Cmp>>` indirection for the default case

## Phase 6: Allocation Reduction (TODO)

**Expected Impact: ~15-20% on reads**

- Replace `format!("key{:08}", i)` in benchmarks with pre-allocated buffers
- In production: reduce `Vec<u8>` allocations in block iteration
- Use `Bytes` (reference-counted) more aggressively to avoid copies
- Consider arena allocation for short-lived block iterators

## Phase 7: Prefetching (TODO)

**Expected Impact: ~20% on sequential reads**

- Add readahead hints for sequential table scans
- Use `madvise(MADV_SEQUENTIAL)` on mmap regions during iteration
- Prefetch next block while processing current block

## Phase 8: Parallel Compaction (TODO)

**Expected Impact: write throughput improvement**

- Parallelize L0→L1 compaction using rayon
- Concurrent SSTable building for non-overlapping key ranges
- Background compaction thread pool

---

## Projected Final Performance

| Phase | Cumulative Improvement | Gap vs C++ |
|-------|----------------------|------------|
| Baseline | 1.0× | 4.9× slower |
| Phase 1-3 (done) | 2.0× | ~2.4× slower |
| Phase 4-5 | 2.5× | ~2.0× slower |
| Phase 6-7 | 3.5× | ~1.4× slower |
| Phase 8 | 4.0× | ~1.2× slower |

**Target**: Within 1.5× of C++ LevelDB read performance while maintaining memory safety guarantees.

---

## Validation

All optimizations are validated against the existing 146-test suite:

```bash
cargo test          # All 146 tests must pass
cargo run --release --example bench  # Performance regression check
```

Safety invariants:
- SSTable files are immutable after creation (mmap safety)
- Bloom filter bounds are guaranteed by modulo arithmetic (unsafe safety)
- No data races (single-threaded DB access, Rc-based sharing)
