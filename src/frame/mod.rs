use std::{fmt, io};

pub mod compress;
pub mod decompress;
pub mod header;

pub use compress::FrameEncoder;
pub use decompress::FrameDecoder;

#[derive(Debug)]
pub enum Error {
    SkippableFrame(u32),
    CompressionError(/* TBD */),
    DecompressionError(crate::block::DecompressError),
    UnimplementedBlocksize(u8),
    UnsupportedVersion(u8),
    IoError(io::Error),
    WrongMagicNumber,
    ReservedBitsSet,
    ContentChecksumError,
    BlockChecksumError,
    HeaderChecksumError,
    BlockTooBig,
    LinkedBlocksNotSupported,
    InvalidBlockInfo,
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
