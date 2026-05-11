```rust
/// Builds an on-disk table file from the provided iterator and updates `meta`.
///
/// This mirrors LevelDB-style table building semantics:
/// - Initializes `meta.file_size` to `0`.
/// - Seeks input iterator to first entry.
/// - If iterator is non-empty, creates a writable table file and appends all entries.
/// - Records smallest/largest keys in `meta`.
/// - Finalizes/syncs/closes the file.
/// - Verifies readability through `table_cache`.
/// - If anything fails (or resulting file is empty), removes the created file.
///
/// Returns an error status on failure.
pub fn build_table(
    dbname: &str,
    env: &mut dyn Env,
    options: &Options,
    table_cache: &mut dyn TableCache,
    iter: &mut dyn DbIterator,
    meta: &mut FileMetaData,
) -> Status {
    fn run(
        dbname: &str,
        env: &mut dyn Env,
        options: &Options,
        table_cache: &mut dyn TableCache,
        iter: &mut dyn DbIterator,
        meta: &mut FileMetaData,
    ) -> Result<(), Status> {
        meta.file_size = 0;
        iter.seek_to_first();

        if !iter.valid() {
            return Ok(());
        }

        let fname = table_file_name(dbname, meta.number);
        let mut file = env.new_writable_file(&fname)?;

        {
            let mut builder = TableBuilder::new(options, &mut *file);

            meta.smallest.decode_from(iter.key());

            let mut last_key = None;
            while iter.valid() {
                let k = iter.key();
                builder.add(k, iter.value());
                last_key = Some(k);
                iter.next();
            }

            if let Some(k) = last_key {
                if !k.is_empty() {
                    meta.largest.decode_from(k);
                }
            }

            builder.finish()?;
            meta.file_size = builder.file_size();
            debug_assert!(meta.file_size > 0);
        }

        file.sync()?;
        file.close()?;

        let it = table_cache.new_iterator(ReadOptions::default(), meta.number, meta.file_size);
        it.status()?;

        if !iter.status().ok() {
            return Err(iter.status());
        }

        Ok(())
    }

    let fname = table_file_name(dbname, meta.number);
    let status = match run(dbname, env, options, table_cache, iter, meta) {
        Ok(()) => Status::ok(),
        Err(e) => e,
    };

    if !(status.ok() && meta.file_size > 0) {
        let _ = env.remove_file(&fname);
    }

    status
}
```