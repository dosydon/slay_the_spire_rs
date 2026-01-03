use serde::{Serialize, Deserialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Entity {
    Enemy(usize),      // Target a specific enemy by index
    Player,           // Target the player
    None,
}
