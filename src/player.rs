extern crate rodio;

use rodio::Sink;

pub struct Player {
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let device =
            rodio::default_output_device().expect("Error: unable to obtain default outut device");
        let sink = Sink::new(&device);

        // Add a dummy source of the sake of the example.
        let source = rodio::source::SineWave::new(440);

        sink.append(source);

        Player { sink }
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
