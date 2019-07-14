use clap::{self, App, Arg, SubCommand};
use play::{Audio, InputHandler, Player, Screen, State};
use rodio;
use std::fs;
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
        .arg(
            Arg::with_name("loop")
                .short("l")
                .long("loop")
                .help("automatically loops through all songs")
                .takes_value(false)
        )
        .subcommand(SubCommand::with_name("devices").about("list all available audio devices"))
        .get_matches();

    let is_loop = matches.is_present("loop");

    // -a option
    if let Some(audio_paths) = matches.values_of("audios") {
        let audios: Vec<Audio> = audio_paths
            .filter_map(|path| {
                let audio = Audio::new(path);

                match audio {
                    Some(_) => (),
                    None => eprintln!("error reading file: {}", path),
                };

                audio
            })
            .collect();

        run(audios, is_loop);
    }

    // -r option
    if let Some(dir) = matches.value_of("read") {
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

        run(audios, is_loop);
    }

    // devices subcommand
    if let Some(_) = matches.subcommand_matches("devices") {
        for device in rodio::devices() {
            println!("name: {}", device.name());
        }
    }
}

fn run(audios: Vec<Audio>, is_loop: bool) {
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

        if is_loop && player.is_empty() {
            state.set_loaded_to_next();
            player.load(state.loaded());
        }

        if !state.is_clean() {
            screen.render(&state);
            state.clean();
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }
}
