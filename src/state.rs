use crate::Audio;

pub struct State {
    audios: Vec<Audio>,
    pointed: usize,
    loaded: usize,
    min: usize,
    max: usize,
}

impl State {
    pub fn new(audios: Vec<Audio>, pointed: usize, loaded: usize, audio_span: usize) -> Self {
        let max = if audios.len() > audio_span {
            audio_span
        } else {
            audios.len()
        };

        State {
            audios,
            pointed,
            loaded,
            min: 0,
            max,
        }
    }

    pub fn audios(&self) -> &[Audio] {
        &self.audios[self.min..self.max]
    }

    pub fn loaded(&self) -> &Audio {
        &self.audios[self.loaded]
    }

    pub fn set_loaded_to_pointed(&mut self) {
        self.loaded = self.pointed;
    }

    pub fn pointed(&self) -> usize {
        self.pointed - self.min
    }

    pub fn prev(&mut self) {
        if self.pointed == self.min {
            if self.min > 0 {
                self.min -= 1;
                self.max -= 1;
                self.pointed -= 1;
            }
        } else {
            self.pointed -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.pointed == self.max - 1 {
            if self.max < self.audios.len() {
                self.min += 1;
                self.max += 1;
                self.pointed += 1;
            }
        } else {
            self.pointed += 1;
        }
    }
}
