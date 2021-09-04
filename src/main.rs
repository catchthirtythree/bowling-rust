mod frame;
mod game;
mod player;
mod scoring;

use crate::game::Game;
use crate::player::Player;

fn main() {
    let num_players = 4;
    let players = vec![Player::default(); num_players];

    let mut game = Game::new(players);

    game.simulate_game();
    // game.show_scores_per_frame();
    game.show_scores_per_player();
}
