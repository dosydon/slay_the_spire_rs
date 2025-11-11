#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Entity {
    Enemy(usize),      // Target a specific enemy by index
    Player,           // Target the player
    None,
}
