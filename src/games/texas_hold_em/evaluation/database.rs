use anyhow::Result;
use std::{fs, path::Path};

pub struct DataBase(pub Vec<u32>);

impl DataBase {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let data = fs::read(path)?;
        let data = Self::format_data(data)?;

        Ok(DataBase(data))
    }

    fn format_data(value: Vec<u8>) -> Result<Vec<u32>> {
        let mut contents = vec![];

        let vec_length = value.len();
        for i in 0..vec_length {
            let range = i * 4..(i * 4) + 4;
            if range.end > vec_length {
                continue;
            }
            let bytes = &value[range].try_into()?;
            contents.push(u32::from_le_bytes(*bytes));
        }

        Ok(contents)
    }
}
