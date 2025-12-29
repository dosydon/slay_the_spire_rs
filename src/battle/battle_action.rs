use crate::battle::target::Entity;

#[derive(Debug, Clone, PartialEq)]
pub enum BattleAction {
    PlayCard(usize, Entity), // Play a card from hand by its index with a target
    SelectCardInHand(usize), // Select a card from hand (for upgrade, put on deck, or duplicate effects)
    SelectCardInDiscard(usize), // Select a card from discard pile
    SelectCardInExhaust(usize), // Select a card from exhaust pile
    SelectCardFromChoices(usize), // Select a card from offered choices (e.g., from Attack Potion)
    UsePotion(usize, Option<Entity>), // Use a potion from inventory by slot index with optional target
    KillAllEnemies,           // Kill all enemies (for easy debugging)
    EndTurn,                 // End the current turn
}