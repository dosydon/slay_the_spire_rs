use crate::game::target::Target;

pub enum Action {
    PlayCard(usize, Target), // Play a card from hand by its index with a target
    EndTurn,                 // End the current turn
}