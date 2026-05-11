use rusty_leveldb::{Options, DB};
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let dir = std::env::temp_dir().join("leveldb_bench_rust");
    let _ = std::fs::remove_dir_all(&dir);

    let mut opts = Options::default();
    opts.create_if_missing = true;
    // Optimized bloom filter: 12 bits/key for lower false positive rate
    opts.filter_policy = Rc::new(Box::new(rusty_leveldb::BloomPolicy::new(12)));

    let mut db = DB::open(&dir, opts).unwrap();
    let n: usize = 1_000_000;

    println!("=== rusty-leveldb Benchmark (1M ops) ===");
    println!("    mmap I/O: enabled (>16KB files)");
    println!("    bloom filter: 12 bits/key");
    println!("    block cache: 32 MB");
    println!();

    // Pre-generate random indices for random reads
    let mut rnd_indices = Vec::with_capacity(n);
    for i in 0..n {
        rnd_indices.push((i * 7 + 13) % n);
    }

    // Pre-generate keys to avoid format! overhead in the hot loop
    let mut keys = Vec::with_capacity(n);
    for i in 0..n {
        keys.push(format!("key{:08}", i).into_bytes());
    }
    let mut vals = Vec::with_capacity(n);
    for i in 0..n {
        vals.push(format!("value{:08}_padding_{}", i, i * 7).into_bytes());
    }

    // --- Sequential Writes ---
    let start = Instant::now();
    for i in 0..n {
        db.put(&keys[i], &vals[i]).unwrap();
    }
    let wd = start.elapsed();
    println!(
        "Seq Writes:       {:.3}s  ({:.0} ops/sec)",
        wd.as_secs_f64(),
        n as f64 / wd.as_secs_f64()
    );

    // Flush to ensure data is on disk for read benchmarks
    db.flush().unwrap();

    // --- Sequential Reads (cold) ---
    let start = Instant::now();
    for i in 0..n {
        let _ = db.get(&keys[i]);
    }
    let rd = start.elapsed();
    println!(
        "Seq Reads (cold): {:.3}s  ({:.0} ops/sec)",
        rd.as_secs_f64(),
        n as f64 / rd.as_secs_f64()
    );

    // --- Sequential Reads (warm cache) ---
    let start = Instant::now();
    for i in 0..n {
        let _ = db.get(&keys[i]);
    }
    let rw = start.elapsed();
    println!(
        "Seq Reads (warm): {:.3}s  ({:.0} ops/sec)",
        rw.as_secs_f64(),
        n as f64 / rw.as_secs_f64()
    );

    // --- Random Reads ---
    let start = Instant::now();
    for &idx in &rnd_indices {
        let _ = db.get(&keys[idx]);
    }
    let rr = start.elapsed();
    println!(
        "Rnd Reads:        {:.3}s  ({:.0} ops/sec)",
        rr.as_secs_f64(),
        n as f64 / rr.as_secs_f64()
    );

    // --- Deletes ---
    let start = Instant::now();
    for i in 0..n / 2 {
        db.delete(&keys[i]).unwrap();
    }
    let dd = start.elapsed();
    println!(
        "Deletes:          {:.3}s  ({:.0} ops/sec)",
        dd.as_secs_f64(),
        (n / 2) as f64 / dd.as_secs_f64()
    );

    // --- Stats ---
    let size: u64 = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum();
    println!("\nDB size: {:.2} MB", size as f64 / 1_048_576.0);

    let _ = std::fs::remove_dir_all(&dir);
}
