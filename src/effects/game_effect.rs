use serde::{Serialize, Deserialize};

/// Game-wide effects that operate outside of battle context
/// These effects modify the player's deck, relics, gold, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEffect {
    // Gold management
    GainGold { amount: u32 },
    SpendGold { amount: u32 },

    // Relic management
    ObtainRandomRelic,

    // Deck modification (with player selection)
    EnterSelectCardsToUpgrade { count: u32 },
    EnterSelectCardsToRemove { count: u32 },
    EnterSelectCardsToTransform { count: u32 },

    // Deck modification (automatic/random)
    UpgradeRandomCards { count: u32 },

    // Event transitions
    TriggerCombatEvent,
}
