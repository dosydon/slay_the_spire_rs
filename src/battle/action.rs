use crate::battle::target::Entity;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlayCard(usize, Entity), // Play a card from hand by its index with a target
    EndTurn,                 // End the current turn
}