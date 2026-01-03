//! Game state types for managing the overall game flow

use crate::game::{shop::ShopState, reward_state::RewardState};
use crate::events::map_events::{MapEvent, EventChoice};
use serde::{Serialize, Deserialize};

/// The overall state of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    /// Player is currently in a battle
    InBattle,
    /// Player is on the map choosing their next path
    OnMap,
    /// Player is viewing rewards after combat (gold, card selection)
    Reward(RewardState),
    /// Player is selecting a card reward from 3 options
    /// Previous state will be on the state stack
    CardRewardSelection(Vec<crate::game::card::Card>),
    /// Player is in an SLS Event making choices
    InEvent(MapEvent, Vec<EventChoice>),
    /// Player is at a rest site
    RestSite,
    /// Player is selecting a card from their deck to upgrade
    SelectingCardFromDeck(CardFromDeckTo),
    /// Player is in a shop
    Shop(ShopState),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardFromDeckTo {
    Upgrade,
    Remove,
}