use clap::{self, App, Arg};
use play::{Audio, InputHandler, Player, State};
use std::{fs, io, io::Write};
use termion::event::Key;
use termion::raw::IntoRawMode;

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
                .takes_value(false),
        )
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
}

fn run(audios: Vec<Audio>, is_loop: bool) {
    let player = Player::new();
    let mut stdout = io::stdout()
        .into_raw_mode()
        .expect("unable to enter terminal raw mode");
    let input_handler = InputHandler::new();
    let mut state = match State::new(audios) {
        Some(state) => state,
        None => return,
    };

    // append the first song
    player.append(state.pointed());
    write_audio(&mut stdout, state.pointed(), &player);

    loop {
        for key in input_handler.keys() {
            match key {
                Key::Char('q') => {
                    write_line(&mut stdout, "Quitting...");
                    return;
                }
                Key::Char(' ') => {
                    player.toggle();
                    write_audio(&mut stdout, state.pointed(), &player);
                }
                _ => (),
            }
        }

        if player.is_empty() {
            if !state.is_pointing_to_last() || (state.is_pointing_to_last() && is_loop) {
                let audio = state.point_to_next().pointed();

                write_audio(&mut stdout, audio, &player);
                player.append(audio);
            } else {
                return;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP_MS));
    }
}

fn write_audio(out: &mut Write, audio: &Audio, player: &Player) {
    let player_status = if player.is_paused() { "||" } else { "|>" };

    write_line(out, &format!("{} {}", player_status, audio.name()));
}

fn write_line(out: &mut Write, msg: &str) {
    writeln!(out, "{}\r", msg).expect("error writing to stdout");
}
