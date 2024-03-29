use crate::Audio;
use rodio::Sink;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let device =
            rodio::default_output_device().expect("Error: unable to obtain default output device");
        let sink = Sink::new(&device);

        sink.pause();

        Player { sink }
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn append(&self, audio: &Audio) {
        self.sink.append(audio.source());
    }

    pub fn toggle(&self) {
        if self.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }
}
