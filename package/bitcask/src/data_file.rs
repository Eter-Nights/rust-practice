use crate::{KeyDir, Result};
use fs4::fs_std::FileExt;
use std::{
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};
const KEY_VAL_HEADER_LEN: u32 = 4;

pub struct DataFile {
    pub path: PathBuf,
    pub file: std::fs::File,
}

impl DataFile {
    pub fn new(path: PathBuf) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        file.try_lock_exclusive()?;

        Ok(Self { path, file })
    }

    // 单 entry 结构如下
    // +-------------+-------------+----------------+----------------+
    // | key len(4)    val len(4)     key(varint)       val(varint)  |
    // +-------------+-------------+----------------+----------------+

    // 构建内存索引
    pub fn load_index(&mut self) -> Result<KeyDir> {
        // 读取 key len(4) 或者 val len(4) 数据
        let mut len_buf = [0u8; KEY_VAL_HEADER_LEN as usize];
        // BTree 内存索引
        let mut keydir = KeyDir::new();
        // 磁盘文件大小
        let file_len = self.file.metadata()?.len();

        // 获取文件读缓冲对象
        let mut r = BufReader::new(&mut self.file);
        // 文件当前偏移量
        let mut pos: u64 = r.seek(SeekFrom::Start(0))?;

        while pos < file_len {
            // 读取 key 的长度
            r.read_exact(&mut len_buf)?;
            let key_len = u32::from_be_bytes(len_buf);

            // 读取 value 的长度
            r.read_exact(&mut len_buf)?;
            let value_lent_or_tomstone = match i32::from_be_bytes(len_buf) {
                l if l >= 0 => Some(l as u32),
                _ => None,
            };
            // 读取 key 的内容
            let mut key = vec![0; key_len as usize];
            r.read_exact(&mut key)?;

            // value 的位置
            let value_pos = pos + KEY_VAL_HEADER_LEN as u64 * 2 + key_len as u64;

            // 跳过 value 的长度
            if let Some(value_len) = value_lent_or_tomstone {
                r.seek_relative(value_len as i64)?;
            }

            match value_lent_or_tomstone {
                Some(value_len) => {
                    keydir.insert(key, (value_pos, value_len));
                    pos = value_pos + value_len as u64;
                }
                None => {
                    keydir.remove(&key);
                    pos = value_pos;
                }
            }
        }

        Ok(keydir)
    }

    // 根据 value 的位置和长度获取 value 的值
    pub fn read_value(&mut self, value_pos: u64, value_len: u32) -> Result<Vec<u8>> {
        let mut value = vec![0; value_len as usize];
        self.file.seek(SeekFrom::Start(value_pos))?;
        self.file.read_exact(&mut value)?;

        Ok(value)
    }

    // 向文件中写入数据
    pub fn write_entry(&mut self, key: &[u8], value: Option<&[u8]>) -> Result<(u64, u32)> {
        let key_len = key.len() as u32;
        let value_len = value.map_or(0, |v| v.len() as u32);
        let value_len_or_tomestone = value.map_or(-1, |v| v.len() as i32);

        // 总共占据的长度
        let len = KEY_VAL_HEADER_LEN * 2 + key_len + value_len;

        let offset = self.file.seek(SeekFrom::End(0))?;
        let mut w = BufWriter::with_capacity(len as usize, &mut self.file);
        w.write_all(&key_len.to_be_bytes())?;
        w.write_all(&value_len_or_tomestone.to_be_bytes())?;
        w.write_all(key)?;
        if let Some(v) = value {
            w.write_all(v)?;
        }
        w.flush()?;

        Ok((offset, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_read_write() -> Result<()> {
        let path = std::env::temp_dir()
            .join("sqldb-disk-engine-log-test1")
            .join("log");

        let mut log = DataFile::new(path.clone())?;
        log.write_entry(b"a", Some(b"val1"))?;
        log.write_entry(b"b", Some(b"val2"))?;
        log.write_entry(b"c", Some(b"val3"))?;

        // rewrite
        log.write_entry(b"a", Some(b"val5"))?;
        // delete
        log.write_entry(b"c", None)?;

        let keydir = log.load_index()?;
        assert_eq!(2, keydir.len());

        path.parent().map(|p| std::fs::remove_dir_all(p));

        Ok(())
    }

    #[test]
    fn test_log_reopen() -> Result<()> {
        let path = std::env::temp_dir()
            .join("sqldb-disk-engine-log-test2")
            .join("log");

        let mut log = DataFile::new(path.clone())?;
        log.write_entry(b"a", Some(b"val1"))?;
        log.write_entry(b"b", Some(b"val2"))?;
        log.write_entry(b"c", Some(b"val3"))?;
        log.write_entry(b"d", Some(b"val4"))?;
        log.write_entry(b"d", None)?;

        drop(log);

        let mut log = DataFile::new(path.clone())?;
        let keydir = log.load_index()?;
        assert_eq!(3, keydir.len());

        path.parent().map(|p| std::fs::remove_dir_all(p));

        Ok(())
    }
}
