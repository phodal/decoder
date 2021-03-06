use crate::dx::command::dexer::process_file_bytes;
use std::path::PathBuf;

pub struct FileBytesConsumer {}

impl FileBytesConsumer {
    pub fn new() -> FileBytesConsumer {
        FileBytesConsumer {}
    }

    pub fn process_file_bytes(&self, name: PathBuf, bytes: Vec<u8>) {
        process_file_bytes(name, bytes);
    }
}
