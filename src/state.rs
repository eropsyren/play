use crate::Audio;

pub struct State {
    audios: Vec<Audio>,
    pointed: usize,
    loaded: usize,
    min: usize,
    max: usize,
    clean: bool,
    paused: bool,
}

impl State {
    pub fn new(audios: Vec<Audio>, pointed: usize, loaded: usize, audio_span: usize, paused: bool) -> Self {
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
            clean: false,
            paused,
        }
    }

    pub fn set_paused(&mut self, value: bool) {
        self.clean = false;
        self.paused = value;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_loaded_to_next(&mut self) {
        self.clean = false;

        if self.loaded == self.audios.len() - 1 {
            self.loaded = 0;
        } else {
            self.loaded += 1;
        }
    }

    pub fn is_clean(&self) -> bool {
        self.clean
    }

    pub fn clean(&mut self) {
        self.clean = true;
    }

    pub fn audios(&self) -> &[Audio] {
        &self.audios[self.min..self.max]
    }

    pub fn loaded(&self) -> &Audio {
        &self.audios[self.loaded]
    }

    pub fn set_loaded_to_pointed(&mut self) {
        self.clean = false;
        self.loaded = self.pointed;
    }

    pub fn pointed(&self) -> usize {
        self.pointed - self.min
    }

    pub fn prev(&mut self) {
        self.clean = false;

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
        self.clean = false;

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
