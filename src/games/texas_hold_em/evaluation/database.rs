pub struct DataBase(pub Vec<u32>);

impl TryFrom<Vec<u8>> for DataBase {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
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
                Err(err) => return Err(format!("Error creating evaluator lookup: {}", err)),
            };
        }

        Ok(DataBase(contents))
    }
}
