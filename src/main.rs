use clap::{self, App, Arg};
use play::{Audio, InputHandler, Player, Screen};
use termion::event::Key;

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
    let player = Player::new();
    let input_handler = InputHandler::new();
    let mut screen = Screen::new();

    screen.clear();
    screen.hide_cursor();

    for audio in &audios {
        player.load(audio);
    }

    'main: loop {
        for key in input_handler.keys() {
            match key {
                Key::Char('q') => break 'main,
                Key::Char(' ') => {
                    if player.is_paused() {
                        player.play();
                    } else {
                        player.pause();
                    }
                }
                _ => (),
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
