use clap::{self, App, Arg, SubCommand};
use play::{Audio, InputHandler, Player, Screen, State};
use std::fs;
use termion::event::Key;
use rodio;

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
                .multiple(true)
                .group("main"),
        )
        .arg(
            Arg::with_name("read")
                .short("r")
                .long("read")
                .value_name("DIR")
                .help("reads a DIR as a list of audios")
                .takes_value(true)
                .group("main"),
        )
        .subcommand(
            SubCommand::with_name("devices")
                .about("list all available audio devices")
        )
        .get_matches();

    let audio_paths = matches.values_of("audios");
    let dir = matches.value_of("read");
    
    if let Some(_) = matches.subcommand_matches("devices") {
        for device in rodio::devices() {
            println!("name: {}", device.name());
        }
    }

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
    } else if dir.is_some() {
        let dir = dir.unwrap();
        let entries = fs::read_dir(dir);

        let audios: Vec<Audio> = match entries {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter_map(|path| match path.to_str() {
                    Some(val) => Some(String::from(val)),
                    None => None,
                })
                .filter_map(|path| Audio::new(&path))
                .collect(),
            Err(_) => {
                eprintln!("error reading directory: {}", dir);
                return;
            }
        };

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
                }
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
