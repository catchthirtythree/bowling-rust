use crate::frame::Frame;
use crate::game::Game;

pub struct Scoring;

impl Scoring {
    pub fn score_frames(frames: &Vec<Frame>) -> u32 {
        let mut score = 0;

        for frame in frames.iter() {
            score += Scoring::score_frame(frame.number, &frames);
        }

        score
    }

    pub fn score_frame(frame_number: usize, frames: &Vec<Frame>) -> u32 {
        let mut score = 0;
        let next_rolls = Scoring::get_next_three_rolls(frame_number, frames);

        if let Some(r1) = next_rolls.get(0) {
            score += r1;

            if *r1 == Game::MAX_ROLL_SCORE {
                if let Some(r2) = next_rolls.get(1) {
                    score += r2;
                }

                if let Some(r3) = next_rolls.get(2) {
                    score += r3;
                }
            } else {
                if let Some(r2) = next_rolls.get(1) {
                    score += r2;

                    if r1 + r2 == Game::MAX_ROLL_SCORE {
                        if let Some(r3) = next_rolls.get(2) {
                            score += r3;
                        }
                    }
                }
            }
        }

        score
    }

    fn get_next_three_rolls(frame_number: usize, frames: &Vec<Frame>) -> Vec<u32> {
        frames[(frame_number - 1)..].iter().fold(vec![], |mut acc, next| {
            for roll in next.rolls.iter() {
                if acc.len() < 3 {
                    acc.push(*roll)
                }
            }

            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::player::Player;

    #[test]
    fn test_calculate_score_to_frame_144() {
        let mut player = Player::default();

        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![6, 3]);

        assert_eq!(Scoring::score_frames(&player.0), 144);
    }

    #[test]
    fn test_calculate_score_120_game() {
        let mut player = Player::default();

        player.add_frame(vec![10]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![2, 1]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![4, 6]);
        player.add_frame(vec![2, 6]);
        player.add_frame(vec![0, 10]);
        player.add_frame(vec![8, 0]);
        player.add_frame(vec![4, 1]);
        player.add_frame(vec![10, 9, 1]);

        assert_eq!(Scoring::score_frames(&player.0), 120);
    }

    #[test]
    fn test_calculate_score_170_game() {
        let mut player = Player::default();

        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3]);
        player.add_frame(vec![7, 3, 7]);

        assert_eq!(Scoring::score_frames(&player.0), 170);
    }

    #[test]
    fn test_calculate_score_perfect_game() {
        let mut player = Player::default();

        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
        player.add_frame(vec![10, 10, 10]);

        assert_eq!(Scoring::score_frames(&player.0), 300);
    }
}