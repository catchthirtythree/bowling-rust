mod frame;
mod game;
mod player;

use crate::game::Game;
use crate::player::Player;

fn main() {
    let num_players = 4;
    let players = vec![Player::default(); num_players];

    let mut game = Game::new(players);

    game.simulate_game();
    game.show_scores();
}

// fn calculate_score(rolls: &mut VecDeque<u32>) -> u32 {
//     let mut current_score: u32 = 0;

//     for current_frame in 1..=MAX_FRAMES_PER_GAME {
//         current_score += calculate_score_for_frame(current_frame, rolls);
//     }

//     current_score
// }

// fn calculate_score_to_frame(frame: u32, rolls: &mut VecDeque<u32>) -> u32 {
//     let mut current_score: u32 = 0;

//     for current_frame in 1..=MAX_FRAMES_PER_GAME {
//         current_score += calculate_score_for_frame(current_frame, rolls);

//         if current_frame == frame {
//             break
//         }
//     }

//     current_score
// }

// fn calculate_score_for_frame(frame: u32, rolls: &mut VecDeque<u32>) -> u32 {
//     let mut frame_score: u32 = 0;

//     if let Some(roll1) = rolls.pop_front() {
//         frame_score += roll1;

//         if roll1 == MAX_ROLL_SCORE {
//             if let Some(b1) = rolls.get(0) {
//                 frame_score += b1;
//             }

//             if let Some(b2) = rolls.get(1) {
//                 frame_score += b2;
//             }
//         } else {
//             if let Some(roll2) = rolls.pop_front() {
//                 frame_score += roll2;

//                 if roll1 + roll2 == MAX_ROLL_SCORE {
//                     if let Some(b1) = rolls.get(0) {
//                         frame_score += b1;
//                     }

//                     // In the final frame, a spare will warrant an extra
//                     // roll for the player
//                     if frame == MAX_FRAMES_PER_GAME {
//                         if let Some(roll3) = rolls.get(0) {
//                             frame_score += roll3;
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     frame_score
// }

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