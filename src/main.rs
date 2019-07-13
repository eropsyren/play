use clap::{self, App, Arg};
use play::{Audio, InputHandler, Player, Screen, State};
use termion::event::Key;

const LOOP_SLEEP_MS: u64 = 50;

fn main() {
    let matches = App::new("play")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("audio player")
        .arg(
            Arg::with_name("audios")
                .short("a")
                .long("audios")
                .value_name("FILEs")
                .help("plays the passed audio FILEs")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let audio_paths = matches.values_of("audios");

    if audio_paths.is_some() {
        let audios: Vec<Audio> = audio_paths
            .unwrap()
            .filter_map(|path| {
                let audio = Audio::new(path);

                match audio {
                    Some(_) => (),
                    None => eprintln!("error reading file: {}", path),
                };

                audio
            })
            .collect();

        run(audios);
    }
}

fn run(audios: Vec<Audio>) {
    let mut player = Player::new();
    let input_handler = InputHandler::new();
    let mut screen = Screen::new();
    let mut state = State::new(audios, 0, 0, 10);

    screen.clear();
    screen.hide_cursor();

    player.load(state.loaded());

    'main: loop {
        for key in input_handler.keys() {
            match key {
                Key::Char('q') => {
                    screen.clear();
                    break 'main;
                }
                Key::Char(' ') => {
                    if player.is_paused() {
                        player.play();
                    } else {
                        player.pause();
                    }
                }
                Key::Up => state.prev(),
                Key::Down => state.next(),
                Key::Char('\n') => {
                    state.set_loaded_to_pointed();
                    player.load(state.loaded());
                },
                _ => (),
            }
        }

        if !state.is_clean() {
            screen.render(&state);
            state.clean();
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }
}
