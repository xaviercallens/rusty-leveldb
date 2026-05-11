fn make_file_name(dbname: &str, number: u64, suffix: &str) -> String {
    format!("{dbname}/{number:06}.{suffix}")
}