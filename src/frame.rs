use crate::game::Game;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Frame {
    pub number: usize,
    pub rolls: Vec<u32>,
}

impl Frame {
    pub fn new(number: usize) -> Frame {
        Frame {
            number,
            rolls: vec![],
        }
    }

    pub fn roll_frame(&mut self) {
        let roll1 = self.roll(Game::MAX_ROLL_SCORE);
        if roll1 == Game::MAX_ROLL_SCORE {
            if self.number == Game::MAX_FRAMES_PER_GAME {
                let roll2 = self.roll(Game::MAX_ROLL_SCORE);
                if roll2 == Game::MAX_ROLL_SCORE {
                    self.roll(Game::MAX_ROLL_SCORE);
                } else {
                    self.roll(Game::MAX_ROLL_SCORE - roll2);
                }
            }
        } else {
            let roll2 = self.roll(Game::MAX_ROLL_SCORE - roll1);
            if self.number == Game::MAX_FRAMES_PER_GAME {
                if (roll1 + roll2) == Game::MAX_ROLL_SCORE {
                    self.roll(Game::MAX_ROLL_SCORE);
                }
            }
        }
    }

    fn roll(&mut self, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let score = rng.gen_range(0..=max);

        self.rolls.push(score);

        score
    }
}
