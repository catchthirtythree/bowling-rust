use crate::player::Player;

#[derive(Debug, PartialEq)]
pub enum GameState {
    /// Waiting for players to join
    Starting,

    /// Game has started; holds the current player index
    Playing(usize),

    /// Game has a resolution
    Done,
}

#[derive(Debug)]
pub enum GameError {
    /// Triggered when trying to start/restart an already started game
    AlreadyStarted,

    /// Triggered when trying to start a game with zero players
    NotEnoughPlayers,
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub players: Vec<Player>
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Starting,
            players: vec![]
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn play(&mut self) {
        match self.state {
            GameState::Starting => {
                println!("Starting game.");
                self.state = GameState::Playing(0);
            },

            GameState::Playing(idx) => {
                let player_number = idx + 1;

                let num_players = self.players.len();
                let player = self.players.get_mut(idx).unwrap();

                player.roll();

                let is_last_player = (player_number) == num_players;
                if is_last_player && player.is_finished_game() {
                    println!("Game is finished.");
                    self.state = GameState::Done;
                } else if player.is_finished_turn() {
                    println!("Changing player.");
                    self.state = GameState::Playing(player_number % num_players)
                }
            },

            GameState::Done => {
                println!("Game is finished.");
            }
        }
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        match self.state {
            GameState::Starting => {
                if self.players.is_empty() {
                    Err(GameError::NotEnoughPlayers)
                } else {
                    Ok(())
                }
            },

            GameState::Playing(_) => {
                Err(GameError::AlreadyStarted)
            },

            GameState::Done => {
                println!("Restarting game.");

                self.state = GameState::Playing(0);

                for player in self.players.iter_mut() {
                    player.reset();
                }

                Ok(())
            }
        }
    }
}