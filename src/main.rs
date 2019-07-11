extern crate play;

use play::Player;

fn main() {
    let player = Player::new();

    while !player.is_empty() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
