```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Unknown,
}

pub fn guess_type(fname: &str) -> Option<FileType> {
    let basename = fname.rsplit('/').next().unwrap_or(fname);
    parse_file_name(basename).map(|(_, file_type)| file_type)
}

fn parse_file_name(_basename: &str) -> Option<(u64, FileType)> {
    None
}
```