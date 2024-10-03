mod data_file;
mod mini_bitcask;

type KeyDir = std::collections::BTreeMap<Vec<u8>, (u64, u32)>;

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub use data_file::DataFile;

pub use mini_bitcask::MiniBitcask;
