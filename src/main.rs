mod game;
mod game_conf;
use game::Game;
use game_conf::GameConf;

use std::{env, fs, process};

fn main() {
    let contents = match read_gamefile() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };
    let conf = GameConf::new(&contents);
    let mut game = Game::new(conf);
    game.start();
}

fn read_gamefile() -> Result<String, String> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        Err(format!("Usage: {} <filename>", args[0]))
    } else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => Ok(contents),
            Err(err) => Err(err.to_string()),
        }
    }
}