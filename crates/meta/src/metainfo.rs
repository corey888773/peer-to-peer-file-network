use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct File {
    pub length: u64,
    pub path: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Mode {
    SingeFile { length: u64 },
    MultiFile { files: Vec<File> },
}

#[derive(Deserialize, Debug)]
pub struct Info {
    // In the single file case, the name key is the name of a file, in the muliple file case, it's the name of a directory.
    pub name: String,
    pub piece_length: u32,
    pub pieces: String,
    // There is also a key length or a key files, but not both or neither.
    // If length is present then the download represents a single file, otherwise it represents a set of files which go in a directory structure.
    pub mode: Mode,
}

#[derive(Deserialize, Debug)]
pub struct MetaFile {
    pub announce: String,
    pub info: Info,
}
