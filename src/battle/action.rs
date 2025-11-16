use crate::battle::target::Entity;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleState {
    PlayerTurn,
    SelectCardInHand,
    SelectCardInDiscard,
    SelectCardInHandToPutOnDeck,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlayCard(usize, Entity), // Play a card from hand by its index with a target
    SelectCardInHand(usize), // Select a card from hand for upgrade effects
    SelectCardInDiscard(usize), // Select a card from discard pile
    EndTurn,                 // End the current turn
}