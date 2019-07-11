use crate::Audio;
use rodio::Sink;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let device =
            rodio::default_output_device().expect("Error: unable to obtain default outut device");
        let sink = Sink::new(&device);

        sink.pause();

        Player { sink }
    }

    pub fn load(&self, audio: &Audio) {
        self.sink.append(audio.source());
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
