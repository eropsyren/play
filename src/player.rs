use crate::Audio;
use rodio::source::Source;
use rodio::{Device, Sink};

pub struct Player {
    device: Device,
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let device =
            rodio::default_output_device().expect("Error: unable to obtain default outut device");
        let sink = Sink::new(&device);

        sink.pause();

        Player { device, sink }
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn load(&mut self, audio: &Audio) {
        if self.sink.empty() {
            self.sink.append(audio.source())
        } else {
            let is_paused = self.is_paused();

            self.sink = Sink::new(&self.device);

            if is_paused {
                self.sink.pause();
            }

            self.sink.append(audio.source());
        }
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }
}
