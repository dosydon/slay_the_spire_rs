#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Target {
    Enemy(usize),      // Target a specific enemy by index
    Player,           // Target the player
    None,
}
