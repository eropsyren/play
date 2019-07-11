use clap::{self, App, Arg};
use play::{Audio, Player};

fn main() {
    let matches = App::new("play")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("audio player")
        .arg(
            Arg::with_name("audio")
                .short("a")
                .long("audio")
                .value_name("FILE")
                .help("plays the aduio FILE")
                .takes_value(true),
        )
        .get_matches();

    let audio = matches.value_of("audio");

    if audio.is_some() {
        let path_to_audio = audio.unwrap();
        let player = Player::new();
        let audio = Audio::new(path_to_audio);

        match audio {
            Some(audio) => {
                player.load(&audio);
                player.play();
            }
            None => eprintln!("error reading file: {}", path_to_audio),
        }

        while !player.is_empty() {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
