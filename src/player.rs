extern crate rodio;

use rodio::Sink;
use crate::Audio;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let device =
            rodio::default_output_device().expect("Error: unable to obtain default outut device");
        let sink = Sink::new(&device);

        Player { sink }
    }

    pub fn load(&self, audio: &Audio) {
        self.sink.append(audio.source());
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
