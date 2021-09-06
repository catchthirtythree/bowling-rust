use crate::{MAX_ROLL_SCORE};
use crate::Frame;

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

            if *r1 == MAX_ROLL_SCORE {
                if let Some(r2) = next_rolls.get(1) {
                    score += r2;
                }

                if let Some(r3) = next_rolls.get(2) {
                    score += r3;
                }
            } else {
                if let Some(r2) = next_rolls.get(1) {
                    score += r2;

                    if r1 + r2 == MAX_ROLL_SCORE {
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
