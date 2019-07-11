use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Song {
    path: Box<Path>,
    title: String,
}

impl Song {
    pub fn new(str_path: &str) -> Option<Self> {
        let path = Path::new(str_path);

        if !is_audio(&Box::from(path)) {
            return None;
        }

        match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => Some(Song {
                    path: Box::from(path),
                    title: String::from(file_name),
                }),
                None => None,
            },
            None => unreachable!(),
        }
    }
}

fn is_audio(path: &Box<Path>) -> bool {
    match File::open(path) {
        Ok(file) => match Decoder::new(BufReader::new(file)) {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
