use dirs;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Audio {
    path: Box<Path>,
    name: String,
}

impl Audio {
    pub fn new(path: &str) -> Option<Self> {
        let path = match dirs::home_dir() {
            Some(path_buf) => match path_buf.to_str() {
                Some(dir) => path.replace("~", dir),
                None => String::from(path),
            },
            None => String::from(path),
        };
        let path = Path::new(&path);

        if !is_audio(&Box::from(path)) {
            return None;
        }

        match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(file_name) => Some(Audio {
                    path: Box::from(path),
                    name: String::from(file_name),
                }),
                None => None,
            },
            None => unreachable!(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> Decoder<BufReader<File>> {
        match File::open(self.path.as_ref()) {
            Ok(file) => match Decoder::new(BufReader::new(file)) {
                Ok(source) => source,
                Err(_) => unreachable!(),
            },
            Err(_) => unreachable!(),
        }
    }
}

fn is_audio(path: &Box<Path>) -> bool {
    match File::open(path) {
        Ok(file) => Decoder::new(BufReader::new(file)).is_ok(),
        Err(_) => false,
    }
}
