use bencoding::{Value, to_value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct File {
    pub length: i64,
    pub path: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum Mode {
    SingeFile { length: i64 },
    MultiFile { files: Vec<File> },
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Info {
    // In the single file case, the name key is the name of a file, in the muliple file case, it's the name of a directory.
    pub name: String,
    pub piece_length: i64,
    pub pieces: String,
    // There is also a key length or a key files, but not both or neither.
    // If length is present then the download represents a single file, otherwise it represents a set of files which go in a directory structure.
    pub mode: Mode,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MetaFile {
    pub announce: String,
    pub info: Info,
}

impl From<&MetaFile> for Value {
    fn from(value: &MetaFile) -> Self {
        to_value(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happypath_bencoded_file_from_meta_file() -> Result<(), Box<dyn std::error::Error>> {
        let _ = Value::from(&MetaFile {
            announce: String::from(""),
            info: Info {
                name: String::from(""),
                piece_length: 10,
                pieces: String::from(""),
                mode: Mode::SingeFile { length: 10 },
            },
        });
        Ok(())
    }
}
