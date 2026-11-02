use std::fs::{DirEntry, ReadDir, read_dir};

pub struct ServiceFile {
    dir: ReadDir,
}

impl ServiceFile {
    pub fn new(path: &String) -> Result<Self, std::io::Error> {
        let dir = read_dir(&path)?;
        Ok(Self { dir })
    }

    pub fn get_number(file_name: &String) -> Option<u32> {
        if let Some(index) = file_name.find(".") {
            let number: u32 = file_name[0..index].parse().unwrap();
            return Some(number);
        }

        None
    }

    pub fn get_artist(file_name: &String) -> Option<String> {
        if let Some(index_one) = file_name.find(".") {
            if let Some(index_two) = file_name.rfind("-") {
                return Some(file_name[index_one..index_two].trim().to_string());
            }
        }

        None
    }

    pub fn get_title(file_name: &String) -> Option<String> {
        if let Some(index_one) = file_name.rfind("-") {
            if let Some(index_two) = file_name.rfind(".") {
                return Some(file_name[index_one..index_two].trim().to_string());
            }
        } else {
            if let Some(index_one) = file_name.find(".") {
                if let Some(index_two) = file_name.rfind(".") {
                    return Some(file_name[index_one + 1..index_two].trim().to_string());
                }
            }
        }

        None
    }
}

impl Iterator for ServiceFile {
    type Item = (DirEntry, String);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.dir.next() {
            if let Ok(file) = file {
                let file_name = file.file_name().to_str().unwrap().to_string();
                if let Some(_) = file_name.find(".mp3") {
                    return Some((file, file_name));
                }
                return None;
            } else {
                None
            }
        } else {
            None
        }
    }
}
