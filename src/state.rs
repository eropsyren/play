use crate::Audio;

pub struct State {
    audios: Vec<Audio>,
    pointed: usize,
}

impl State {
    pub fn new(audios: Vec<Audio>) -> Option<Self> {
        if audios.len() == 0 {
            None
        } else {
            Some(State { audios, pointed: 0 })
        }
    }

    pub fn is_pointing_to_last(&self) -> bool {
        self.audios.len() - 1 == self.pointed
    }

    pub fn point_to_next(&mut self) -> &Self {
        if self.pointed < self.audios.len() {
            self.pointed += 1;
        } else {
            self.pointed = 0;
        }

        self
    }

    pub fn point_to_first(&mut self) -> &Self {
        self.pointed = 0;

        self
    }

    pub fn pointed(&self) -> &Audio {
        &self.audios[self.pointed]
    }
}
