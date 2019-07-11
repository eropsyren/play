use std::sync::mpsc::{self, Receiver, TryIter};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

const INPUT_THREAD_SLEEP_MS: u64 = 50;

pub struct InputHandler {
    keys_reciever: Receiver<Key>,
}

impl InputHandler {
    pub fn new() -> Self {
        let (keys_sender, keys_reciever) = mpsc::channel();
        let stdin = termion::async_stdin();
        let mut stdin = stdin.keys();

        thread::spawn(move || loop {
            let key = stdin.next();

            match key {
                Some(Ok(key)) => keys_sender.send(key).unwrap(),
                _ => thread::sleep(Duration::from_millis(INPUT_THREAD_SLEEP_MS)),
            }
        });

        InputHandler { keys_reciever }
    }

    pub fn keys(&self) -> TryIter<'_, Key> {
        self.keys_reciever.try_iter()
    }
}
