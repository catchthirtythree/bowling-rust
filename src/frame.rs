use crate::{MAX_FRAMES_PER_GAME, MAX_ROLL_SCORE};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub number: usize,
    pub rolls: Vec<u32>
}

impl Frame {
    pub fn new(number: usize) -> Frame {
        Frame {
            number,
            rolls: vec![]
        }
    }

    pub fn is_finished(&self) -> bool {
        if self.number == MAX_FRAMES_PER_GAME {
            self.is_finished_last_frame()
        } else {
            self.is_finished_regular_frame()
        }
    }

    fn is_finished_regular_frame(&self) -> bool {
        match self.rolls.len() {
            0 => false,
            1 => self.rolls[0] == MAX_ROLL_SCORE,
            2 => true,
            _ => panic!("More than 2 rolls in a regular frame??")
        }
    }

    fn is_finished_last_frame(&self) -> bool {
        match self.rolls.len() {
            0 => false,
            1 => false,
            2 => {
                let has_no_strike = self.rolls[0] != MAX_ROLL_SCORE;
                let has_no_spare = (self.rolls[0] + self.rolls[1]) != MAX_ROLL_SCORE;

                has_no_strike && has_no_spare
            },
            3 => true,
            _ => panic!("More than 3 rolls in a final frame??")
        }
    }

    pub fn roll(&mut self) {
        if self.number == MAX_FRAMES_PER_GAME {
            self.roll_last_frame()
        } else {
            self.roll_regular_frame()
        }
    }

    fn roll_regular_frame(&mut self) {
        let mut rng = rand::thread_rng();

        match self.rolls.len() {
            0 => {
                let roll = rng.gen_range(0..=MAX_ROLL_SCORE);
                self.rolls.push(roll);
            },

            1 => {
                let score = self.rolls.iter().fold(0, |acc, next| acc + next);

                assert!(score != MAX_ROLL_SCORE);

                let roll = rng.gen_range(0..=(MAX_ROLL_SCORE - score));
                self.rolls.push(roll);
            },

            _ => panic!("No.")
        }
    }

    fn roll_last_frame(&mut self) {
        let mut rng = rand::thread_rng();

        match self.rolls.len() {
            0 => {
                let roll = rng.gen_range(0..=MAX_ROLL_SCORE);
                self.rolls.push(roll);
            },

            1 => {
                let score = self.rolls.iter().fold(0, |acc, next| acc + next);
                if score == MAX_ROLL_SCORE {
                    let roll = rng.gen_range(0..=MAX_ROLL_SCORE);
                    self.rolls.push(roll);
                } else {
                    let roll = rng.gen_range(0..=(MAX_ROLL_SCORE - score));
                    self.rolls.push(roll);
                }
            },

            2 => {
                let score = self.rolls.iter().fold(0, |acc, next| acc + next);
                if score >= MAX_ROLL_SCORE {
                    let roll = rng.gen_range(0..=MAX_ROLL_SCORE);
                    self.rolls.push(roll);
                }
            },

            _ => panic!("No.")
        }
    }
}