mod card;
mod game;
mod hand_rank;
mod player;

use game::Game;

fn main() {
    let mut game = Game::new(1);
    game.play_round();
}
