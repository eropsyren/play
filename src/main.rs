use clap::{self, App, Arg};
use play::{Audio, Player};

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
    player.play();

    for audio in &audios {
        player.load(audio);
    }

    while !player.is_empty() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
