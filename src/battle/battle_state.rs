use crate::game::card_enum::CardEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BattleState {
    PlayerTurn,
    SelectCardInHand (CardInHandTo),
    SelectCardInDiscard,
    SelectCardInExhaust,
    SelectCardFromChoices {
        /// The cards to choose from
        choices: Vec<CardEnum>,
        /// Number of copies to add when selected
        num_copies: u32,
        /// Cost override for the added cards (None = keep original cost)
        cost_override: Option<u32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardInHandTo {
    PutOnDeck,
    Upgrade,
    Duplicate { copies: u32 },
}