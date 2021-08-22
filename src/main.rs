mod game;
mod game_conf;
use game::Game;
use game_conf::GameConf;

use std::fs;

fn main() {
    let filename = "game.txt";
    let contents =
        fs::read_to_string(filename).expect(format!("file `{}` not found", filename).as_str());
    let conf = GameConf::new(&contents);
    let mut game = Game::new(conf);
    game.start();
}
