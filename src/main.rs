use std::io;

use chess::{ai::AiPlayer, cli::InteractiveCliPlayer, play_chess, Player};

extern crate clap;
use clap::{App, Arg};

fn to_player(config_string: &str) -> Box<dyn Player> {
    match config_string {
        "cli" => Box::new(InteractiveCliPlayer::new()),
        "gui" => todo!(),
        "ai1" | "ai2" | "ai3" => {
            Box::new(AiPlayer::new(config_string.as_bytes()[2] as u8 - '0' as u8))
        }
        _ => Box::new(AiPlayer::new(3)),
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("Chess")
        .version("0.1.0")
        .author("Joe Armitage <joe@armitage.com>")
        .about("A Chess game written entirely in Rust")
        .arg(
            Arg::with_name("white")
                .short("w")
                .long("white")
                .value_name("PLAYER_TYPE=[ai[1-5]|cli|gui]")
                .help("Sets the player type for the white player")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("black")
                .short("b")
                .long("black")
                .value_name("PLAYER_TYPE=[ai[1-5]|cli|gui]")
                .help("Sets the player type for the black player")
                .takes_value(true),
        )
        .get_matches();
    let white_player_config = matches.value_of("white").unwrap_or("cli");
    let black_player_config = matches.value_of("black").unwrap_or("ai2");
    play_chess(
        &(*to_player(white_player_config)),
        &(*to_player(black_player_config)),
    )?;
    Ok(())
}
