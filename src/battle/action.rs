use crate::battle::target::Entity;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleState {
    PlayerTurn,
    SelectCardInHand,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlayCard(usize, Entity), // Play a card from hand by its index with a target
    SelectCardInHand(usize), // Select a card from hand for upgrade effects
    EndTurn,                 // End the current turn
}