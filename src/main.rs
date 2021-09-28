use std::io;

use chess::{ai::AiPlayer, cli::InteractiveCliPlayer, play_chess};

fn main() -> io::Result<()> {
    let white_player = InteractiveCliPlayer::new();
    let black_player = AiPlayer::new(3);
    play_chess(&white_player, &black_player)
}
