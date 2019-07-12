use crate::State;
use std::io::{self, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
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

    pub fn render(&mut self, state: &State) {
        self.term
            .draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                    .split(f.size());

                let text = [Text::styled(
                    format!("currently loaded: {}\n", state.loaded().name()),
                    Style::default().fg(Color::Green),
                )];

                Paragraph::new(text.iter())
                    .block(Block::default().borders(Borders::NONE))
                    .alignment(Alignment::Left)
                    .render(&mut f, chunks[0]);

                let text: Vec<Text> = state
                    .audios()
                    .iter()
                    .enumerate()
                    .map(|(idx, audio)| {
                        let name = format!("{}\n", audio.name());

                        if idx == state.pointed() {
                            Text::styled(name, Style::default().fg(Color::Green))
                        } else {
                            Text::raw(name)
                        }
                    })
                    .collect();

                Paragraph::new(text.iter())
                    .block(Block::default().borders(Borders::NONE))
                    .alignment(Alignment::Left)
                    .render(&mut f, chunks[1]);
            })
            .expect("error rendering application");
    }
}
