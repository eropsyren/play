use std::io::{self, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::Terminal;

pub struct Screen {
    term: Terminal<TermionBackend<RawTerminal<Stdout>>>,
}

impl Screen {
    pub fn new() -> Self {
        let stdout = io::stdout()
            .into_raw_mode()
            .ok()
            .expect("unable to enter raw mode");
        let backend = TermionBackend::new(stdout);
        let term = Terminal::new(backend)
            .ok()
            .expect("unable to create terminal");

        Screen { term }
    }

    pub fn clear(&mut self) {
        self.term.clear().expect("error clearing the screen");
    }

    pub fn hide_cursor(&mut self) {
        self.term.hide_cursor().expect("unable to hide cursor");
    }

    pub fn render(&mut self) {
        self.term
            .draw(|_f| {})
            .expect("error rendering application");
    }
}
