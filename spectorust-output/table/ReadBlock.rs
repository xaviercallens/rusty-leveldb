```rust
const K_BLOCK_TRAILER_SIZE: usize = 5;
const K_NO_COMPRESSION: u8 = 0;
const K_SNAPPY_COMPRESSION: u8 = 1;
const K_ZSTD_COMPRESSION: u8 = 2;

/// Options controlling read behavior.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReadOptions {
    pub verify_checksums: bool,
}

/// Location and size of a block in a file.
#[derive(Debug, Clone, Copy)]
pub struct BlockHandle {
    offset: u64,
    size: u64,
}

impl BlockHandle {
    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

/// Returned block payload and cache metadata.
#[derive(Debug, Clone, Default)]
pub struct BlockContents {
    pub data: Vec<u8>,
    pub cachable: bool,
    pub heap_allocated: bool,
}

/// Error type for block reads.
#[derive(Debug, thiserror::Error)]
pub enum ReadBlockError {
    #[error("io error: {0}")]
    Io(String),
    #[error("corruption: {0}")]
    Corruption(&'static str),
}

/// Random-access file abstraction.
pub trait RandomAccessFile {
    /// Reads exactly `len` bytes starting at `offset`.
    fn read_exact_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, ReadBlockError>;
}

/// Reads a little-endian fixed32 from `src` (must be at least 4 bytes).
fn decode_fixed32(src: &[u8]) -> u32 {
    u32::from_le_bytes(src[..4].try_into().unwrap())
}

/// LevelDB-style crc32c unmask.
fn crc32c_unmask(masked_crc: u32) -> u32 {
    const K_MASK_DELTA: u32 = 0xa282ead8;
    let rot = masked_crc.wrapping_sub(K_MASK_DELTA);
    (rot >> 17) | (rot << 15)
}

/// Computes crc32c over `data`.
fn crc32c_value(data: &[u8]) -> u32 {
    crc32c::crc32c(data)
}

/// Reads and decodes a block (optionally verifying checksum and decompressing).
pub fn read_block(
    file: &dyn RandomAccessFile,
    options: ReadOptions,
    handle: BlockHandle,
) -> Result<BlockContents, ReadBlockError> {
    let n = usize::try_from(handle.size()).map_err(|_| ReadBlockError::Corruption("block too large"))?;
    let total = n
        .checked_add(K_BLOCK_TRAILER_SIZE)
        .ok_or(ReadBlockError::Corruption("block size overflow"))?;

    let mut contents = file.read_exact_at(handle.offset(), total)?;
    if contents.len() != total {
        return Err(ReadBlockError::Corruption("truncated block read"));
    }

    let compression = contents[n];
    let trailer = &contents[n..n + K_BLOCK_TRAILER_SIZE];

    if options.verify_checksums {
        let expected = crc32c_unmask(decode_fixed32(&trailer[1..]));
        let actual = crc32c_value(&contents[..n + 1]);
        if actual != expected {
            return Err(ReadBlockError::Corruption("block checksum mismatch"));
        }
    }

    let data = match compression {
        K_NO_COMPRESSION => {
            contents.truncate(n);
            contents
        }
        K_SNAPPY_COMPRESSION => {
            let decoded = snap::raw::Decoder::new()
                .decompress_vec(&contents[..n])
                .map_err(|_| ReadBlockError::Corruption("snappy decompression failed"))?;
            decoded
        }
        K_ZSTD_COMPRESSION => zstd::stream::decode_all(&contents[..n])
            .map_err(|_| ReadBlockError::Corruption("zstd decompression failed"))?,
        _ => return Err(ReadBlockError::Corruption("bad block compression type")),
    };

    Ok(BlockContents {
        data,
        cachable: true,
        heap_allocated: true,
    })
}
```