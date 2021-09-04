use crate::frame::Frame;
use crate::game::Game;

#[derive(Clone, Debug)]
pub struct Player(pub Vec<Frame>);

impl Player {
    pub fn add_frame(&mut self, rolls: Vec<u32>) {
        if self.0.len() == Game::MAX_FRAMES_PER_GAME {
            panic!("This player has already played a full game.");
        }

        let frame_number = self.0.len() + 1;
        let frame = Frame::new(frame_number, rolls);

        self.0.push(frame);
    }
}

impl Default for Player {
    fn default() -> Player {
        Player(vec![])
    }
}
