#[derive(Debug, Clone, PartialEq)]
pub enum BattleError {
    InvalidAction,
    NotEnoughEnergy,
    CardNotInHand,
    InvalidTarget,
    GameAlreadyOver,
    CardNotPlayable,
    CardNotInDiscardPile,
    PotionNotInInventory(usize)
}
