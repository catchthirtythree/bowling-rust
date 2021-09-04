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

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_calculate_score_to_frame_135() {
    //     let rolls = [10, 10, 10, 10, 10, 6, 3];

    //     assert_eq!(calculate_score_to_frame(5, &mut VecDeque::from_iter(rolls)), 135);
    // }

    // #[test]
    // fn test_calculate_score_120_game() {
    //     let rolls = [10, 7, 3, 2, 1, 7, 3, 4, 6, 2, 6, 0, 10, 8, 0, 4, 1, 10, 9, 1];

    //     assert_eq!(calculate_score(&mut VecDeque::from_iter(rolls)), 120);
    // }

    // #[test]
    // fn test_calculate_score_179_game() {
    //     let rolls = [7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7, 3, 7];

    //     assert_eq!(calculate_score(&mut VecDeque::from_iter(rolls)), 177);
    // }

    // #[test]
    // fn test_calculate_score_perfect_game() {
    //     let rolls = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];

    //     assert_eq!(calculate_score(&mut VecDeque::from_iter(rolls)), 300);
    // }
}