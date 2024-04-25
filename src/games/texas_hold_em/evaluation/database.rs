use std::{fs, path::Path};

pub struct DataBase(pub Vec<u32>);

impl DataBase {
    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        let data = match fs::read(path) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error loading evaluator data file: {}", err)),
        };

        let data = match Self::format_data(data) {
            Ok(data) => data,
            Err(err) => return Err(format!("Error parsing evaluator data file: {}", err)),
        };

        Ok(DataBase(data))
    }

    fn format_data(value: Vec<u8>) -> Result<Vec<u32>, String> {
        let mut contents = vec![];

        let vec_length = value.len();
        for i in 0..vec_length {
            let range = i * 4..(i * 4) + 4;
            if range.end > vec_length {
                continue;
            }
            let bytes = &value[range].try_into();
            match bytes {
                Ok(bytes) => contents.push(u32::from_le_bytes(*bytes)),
                Err(err) => return Err(format!("Error parsing evaluator data: {}", err)),
            };
        }

        Ok(contents)
    }
}
