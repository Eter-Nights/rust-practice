use crate::data_file::DataFile;
use crate::{KeyDir, Result};
use std::collections::btree_map;
use std::ops::Bound;
use std::path::PathBuf;

const MERGE_FILE_EXT: &str = "merge";
pub struct MiniBitcask {
    data: DataFile,
    keydir: KeyDir,
}

impl Drop for MiniBitcask {
    fn drop(&mut self) {
        if let Err(error) = self.flush() {
            log::error!("failed to flush file: {:?}", error)
        }
    }
}

impl MiniBitcask {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut data = DataFile::new(path)?;
        let keydir = data.load_index()?;

        Ok(Self { data, keydir })
    }

    pub fn merge(&mut self) -> Result<()> {
        // 创建一个新的临时文件用于写入
        let mut merge_path = self.data.path.clone();
        merge_path.set_extension(MERGE_FILE_EXT);

        let mut new_data = DataFile::new(merge_path)?;
        let mut new_keydir = KeyDir::new();

        // 重写数据
        for (key, (value_pos, value_len)) in self.keydir.iter() {
            let value = self.data.read_value(*value_pos, *value_len)?;
            let (offset, len) = new_data.write_entry(key, Some(&value))?;

            new_keydir.insert(
                key.clone(),
                (offset + len as u64 - *value_len as u64, *value_len),
            );
        }

        // 重写完成，重命名文件
        std::fs::remove_file(self.data.path.clone())?;
        std::fs::rename(new_data.path, self.data.path.clone())?;

        new_data.path = self.data.path.clone();
        // 替换现在的
        self.data = new_data;
        self.keydir = new_keydir;

        Ok(())
    }

    pub fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        let (offset, len) = self.data.write_entry(key, Some(&value))?;
        let value_len = value.len() as u32;
        self.keydir.insert(
            key.to_vec(),
            (offset + len as u64 - value_len as u64, value_len),
        );

        Ok(())
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        if let Some((value_pos, value_len)) = self.keydir.get(key) {
            let value = self.data.read_value(*value_pos, *value_len)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.data.write_entry(key, None)?;
        self.keydir.remove(key);
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(self.data.file.sync_all()?)
    }

    pub fn scan(&mut self, range: impl std::ops::RangeBounds<Vec<u8>>) -> ScanIterator<'_> {
        ScanIterator {
            inner: self.keydir.range(range),
            data: &mut self.data,
        }
    }

    pub fn scan_prefix(&mut self, prefix: &[u8]) -> ScanIterator<'_> {
        let start = Bound::Included(prefix.to_vec());

        // 最后一位加一，例如原始前缀是 "aaaa"，变为 "aaab"
        let mut bound_prefix = prefix.to_vec().clone();
        if let Some(last) = bound_prefix.iter_mut().last() {
            *last += 1;
        }
        let end = Bound::Excluded(bound_prefix);

        self.scan((start, end))
    }
}

pub struct ScanIterator<'a> {
    inner: btree_map::Range<'a, Vec<u8>, (u64, u32)>,
    data: &'a mut DataFile,
}

impl<'a> ScanIterator<'a> {
    fn map(&mut self, item: (&Vec<u8>, &(u64, u32))) -> <Self as Iterator>::Item {
        let (key, (value_pos, value_len)) = item;
        let value = self.data.read_value(*value_pos, *value_len)?;

        Ok((key.clone(), value))
    }
}

impl<'a> Iterator for ScanIterator<'a> {
    type Item = Result<(Vec<u8>, Vec<u8>)>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|item| self.map(item))
    }
}

impl<'a> DoubleEndedIterator for ScanIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|item| self.map(item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Bound;

    #[test]
    fn test_point_opt() -> Result<()> {
        let path = std::env::temp_dir().join("minibitcask-test").join("log");
        let mut eng = MiniBitcask::new(path.clone())?;

        // 测试获取一个不存在的 key
        assert_eq!(eng.get(b"not exist")?, None);

        // 获取一个存在的 key
        eng.set(b"aa", vec![1, 2, 3, 4])?;
        assert_eq!(eng.get(b"aa")?, Some(vec![1, 2, 3, 4]));

        // 重复 put，将会覆盖前一个值
        eng.set(b"aa", vec![5, 6, 7, 8])?;
        assert_eq!(eng.get(b"aa")?, Some(vec![5, 6, 7, 8]));

        // 删除之后再读取
        eng.delete(b"aa")?;
        assert_eq!(eng.get(b"aa")?, None);

        // key、value 为空的情况
        assert_eq!(eng.get(b"")?, None);
        eng.set(b"", vec![])?;
        assert_eq!(eng.get(b"")?, Some(vec![]));

        eng.set(b"cc", vec![5, 6, 7, 8])?;
        assert_eq!(eng.get(b"cc")?, Some(vec![5, 6, 7, 8]));

        path.parent().map(|p| std::fs::remove_dir_all(p));
        Ok(())
    }

    // 测试扫描
    #[test]
    fn test_scan() -> Result<()> {
        let path = std::env::temp_dir()
            .join("minibitcask-scan-test")
            .join("log");
        let mut eng = MiniBitcask::new(path.clone())?;

        eng.set(b"nnaes", b"value1".to_vec())?;
        eng.set(b"amhue", b"value2".to_vec())?;
        eng.set(b"meeae", b"value3".to_vec())?;
        eng.set(b"uujeh", b"value4".to_vec())?;
        eng.set(b"anehe", b"value5".to_vec())?;

        let start = Bound::Included(b"a".to_vec());
        let end = Bound::Excluded(b"e".to_vec());

        let mut iter = eng.scan((start.clone(), end.clone()));
        let (key1, _) = iter.next().expect("no value founded")?;
        assert_eq!(key1, b"amhue".to_vec());

        let (key2, _) = iter.next().expect("no value founded")?;
        assert_eq!(key2, b"anehe".to_vec());
        drop(iter);

        let start = Bound::Included(b"b".to_vec());
        let end = Bound::Excluded(b"z".to_vec());
        let mut iter2 = eng.scan((start, end));

        let (key3, _) = iter2.next_back().expect("no value founded")?;
        assert_eq!(key3, b"uujeh".to_vec());

        let (key4, _) = iter2.next_back().expect("no value founded")?;
        assert_eq!(key4, b"nnaes".to_vec());

        let (key5, _) = iter2.next_back().expect("no value founded")?;
        assert_eq!(key5, b"meeae".to_vec());

        path.parent().map(|p| std::fs::remove_dir_all(p));
        Ok(())
    }

    // 测试前缀扫描
    #[test]
    fn test_scan_prefix() -> Result<()> {
        let path = std::env::temp_dir()
            .join("minibitcask-scan-prefix-test")
            .join("log");
        let mut eng = MiniBitcask::new(path.clone())?;

        eng.set(b"ccnaes", b"value1".to_vec())?;
        eng.set(b"camhue", b"value2".to_vec())?;
        eng.set(b"deeae", b"value3".to_vec())?;
        eng.set(b"eeujeh", b"value4".to_vec())?;
        eng.set(b"canehe", b"value5".to_vec())?;
        eng.set(b"aanehe", b"value6".to_vec())?;

        let prefix = b"ca";
        let mut iter = eng.scan_prefix(prefix);
        let (key1, _) = iter.next().transpose()?.unwrap();
        assert_eq!(key1, b"camhue".to_vec());
        let (key2, _) = iter.next().transpose()?.unwrap();
        assert_eq!(key2, b"canehe".to_vec());

        println!("{:?}", path.clone());
        path.parent().map(|p| std::fs::remove_dir_all(p));
        Ok(())
    }

    #[test]
    fn test_merge() -> Result<()> {
        let path = std::env::temp_dir()
            .join("minibitcask-merge-test")
            .join("log");

        let mut eng = MiniBitcask::new(path.clone())?;

        eng.set(b"a", b"value1".to_vec())?;
        eng.set(b"b", b"value2".to_vec())?;
        eng.set(b"c", b"value3".to_vec())?;
        eng.delete(b"a")?;
        eng.delete(b"b")?;
        eng.delete(b"c")?;

        eng.merge()?;

        eng.set(b"a", b"value1".to_vec())?;
        eng.set(b"b", b"value2".to_vec())?;
        eng.set(b"c", b"value3".to_vec())?;

        let val = eng.get(b"a")?;
        assert_eq!(b"value1".to_vec(), val.unwrap());

        let val = eng.get(b"b")?;
        assert_eq!(b"value2".to_vec(), val.unwrap());

        let val = eng.get(b"c")?;
        assert_eq!(b"value3".to_vec(), val.unwrap());

        path.parent().map(|p| std::fs::remove_dir_all(p));
        Ok(())
    }
}
