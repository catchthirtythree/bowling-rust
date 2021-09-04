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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_frame() {
        let mut player = Player::default();

        player.add_frame(vec![10]);

        assert_eq!(player.0[0].number, 1);
        assert_eq!(player.0[0].rolls, vec![10]);
    }

    #[test]
    #[should_panic(expected = "This player has already played a full game.")]
    fn test_add_too_many_frames() {
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
        player.add_frame(vec![10]);
        player.add_frame(vec![10]);
    }

    #[test]
    fn test_default() {
        let player = Player::default();

        assert_eq!(player.0.len(), 0);
    }
}