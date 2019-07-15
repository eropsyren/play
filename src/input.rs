use std::sync::mpsc::{self, Receiver, TryIter};
use std::{io, thread};
use termion::event::Key;
use termion::input::TermRead;

pub struct InputHandler {
    keys_reciever: Receiver<Key>,
}

impl InputHandler {
    pub fn new() -> Self {
        let (keys_sender, keys_reciever) = mpsc::channel();
        let stdin = io::stdin();

        thread::spawn(move || {
            for key in stdin.keys() {
                match key {
                    Ok(key) => keys_sender.send(key).unwrap(),
                    _ => (),
                }
            }
        });

        InputHandler { keys_reciever }
    }

    pub fn keys(&self) -> TryIter<'_, Key> {
        self.keys_reciever.try_iter()
    }
}
