# Intense Benchmark: C++ LevelDB vs rusty-leveldb (1,000,000 Ops)

This document contains a direct comparison between the upstream Google C++ LevelDB implementation and `rusty-leveldb` after applying the Phase 3 Socrate optimizations (Bloom filter fast-paths and fixed32 encoding).

**Hardware:** Apple Silicon (M-series)
**Dataset:** 1,000,000 Keys
**Configuration:**
- Bloom Filter: 12 bits/key
- Block Cache: 32 MB
- MMap I/O Enabled

## Throughput Comparison (Ops/sec)

| Operation | C++ LevelDB (v1.23) | rusty-leveldb (v4.0.1) | Difference |
|-----------|---------------------|------------------------|------------|
| **Sequential Writes** | 750,750 | 628,317 | Rust is ~16% slower |
| **Random Reads** | 742,942 | 336,353 | Rust is ~55% slower |
| **Sequential Deletes**| 838,222 | 896,363 | **Rust is ~7% FASTER** |

## Latency Comparison (Microseconds/Op)

| Operation | C++ LevelDB | rusty-leveldb |
|-----------|-------------|---------------|
| **Sequential Writes** | 1.332 µs | 1.592 µs |
| **Random Reads** | 1.346 µs | 2.973 µs |
| **Sequential Deletes**| 1.193 µs | 1.116 µs |

## Analysis & Next Steps

1. **Write Performance**: The Rust implementation is approaching parity with C++. The remaining 16% gap is largely attributed to `memcmp` bounds checking in the `SkipMap` when inserting into the MemTable.
2. **Delete Performance**: Rust is marginally faster at batch deletes, likely due to more efficient allocation strategies in the Rust standard library for small `Vec` operations during MemTable tombstone inserts.
3. **Read Performance (The Gap)**: The random read performance is currently the largest bottleneck (336K vs 742K). This is explicitly addressed in our optimization roadmap:
   - **Phase 4 & 5 (Iterator Devirtualization)**: `rusty-leveldb` currently relies heavily on `Box<dyn LdbIterator>`, incurring virtual method call overhead on every block traversal.
   - **Phase 6 (Allocation Reduction)**: C++ LevelDB utilizes `std::string` which uses Small String Optimization (SSO), avoiding heap allocations for small values. `rusty-leveldb` currently allocates a new `Vec<u8>` on the heap for every `db.get()` request. Transitioning to zero-copy slices will significantly narrow this gap.
