use crate::game::Game;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Frame {
    pub number: usize,
    pub rolls: Vec<u32>,
}

impl Frame {
    pub fn new(number: usize, rolls: Vec<u32>) -> Frame {
        Frame { number, rolls }
    }

    pub fn roll_frame(number: usize) -> Frame {
        let mut rolls = vec![];

        let roll1 = Frame::roll(Game::MAX_ROLL_SCORE, &mut rolls);
        if roll1 == Game::MAX_ROLL_SCORE {
            if number == Game::MAX_FRAMES_PER_GAME {
                let roll2 = Frame::roll(Game::MAX_ROLL_SCORE, &mut rolls);
                if roll2 == Game::MAX_ROLL_SCORE {
                    Frame::roll(Game::MAX_ROLL_SCORE, &mut rolls);
                } else {
                    Frame::roll(Game::MAX_ROLL_SCORE - roll2, &mut rolls);
                }
            }
        } else {
            let roll2 = Frame::roll(Game::MAX_ROLL_SCORE - roll1, &mut rolls);
            if number == Game::MAX_FRAMES_PER_GAME {
                if (roll1 + roll2) == Game::MAX_ROLL_SCORE {
                    Frame::roll(Game::MAX_ROLL_SCORE, &mut rolls);
                }
            }
        }

        Frame::new(number, rolls)
    }

    fn roll(max: u32, rolls: &mut Vec<u32>) -> u32 {
        let mut rng = rand::thread_rng();
        let score = rng.gen_range(0..=max);

        rolls.push(score);

        score
    }
}
