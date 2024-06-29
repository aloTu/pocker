mod card;
mod game;
mod hand_rank;
mod player;

use game::Game;

struct Config {
    small_blind: u32,
    initial_chips: u32,
}

const CONFIG: Config = Config {
    small_blind: 10,
    initial_chips: 1000,
};

fn main() {
    let mut game = Game::new(1, CONFIG.initial_chips);
    game.play_round();
}
