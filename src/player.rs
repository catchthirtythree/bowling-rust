use crate::frame::Frame;

#[derive(Clone, Debug)]
pub struct Player(pub Vec<Frame>);

impl Default for Player {
    fn default() -> Player {
        Player(vec![])
    }
}
