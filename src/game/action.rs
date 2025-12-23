//! Global game actions that can be performed throughout the entire game

use bevy::prelude::Event;
use crate::battle::battle_action::BattleAction as BattleAction;

/// High-level game actions that encompass the entire game flow
#[derive(Debug, Clone, PartialEq, Event)]
pub enum GameAction {
    /// Execute a battle action (PlayCard, EndTurn, etc.)
    /// Only valid when in battle
    Battle(BattleAction),

    /// Choose a path on the map (0-based index)
    /// Determines what type of encounter comes next
    ChoosePath(usize),

    /// Claim gold reward from combat
    /// Only valid when in Reward state with unclaimed gold
    ClaimGold,

    /// Claim potion reward from combat
    /// Only valid when in Reward state with unclaimed potion
    ClaimPotion,

    /// Claim relic reward from treasure chest
    /// Only valid when in Reward state with unclaimed relic
    ClaimRelic,

    /// Request card selection (transitions from Reward state to CardRewardSelection state)
    /// Only valid when in Reward state with card_selection_available = true
    RequestCardSelection,

    /// Skip current interaction (rewards, shop, etc.) and return to map
    /// Valid when in Reward state or Shop state
    Skip,

    /// Select a card reward (0, 1, or 2)
    /// Only valid when in CardRewardSelection state
    SelectCardReward(usize),

    /// Make a choice in an SLS Event (0-based index)
    /// Only valid when in an SLS Event
    ChooseEvent(usize),

    /// Make a choice at a rest site (0-based index)
    /// Only valid when at a rest site
    RestSiteChoice(RestSiteAction),

    /// Select a card from deck to upgrade (card index in deck)
    /// Only valid when in SelectUpgradeFromDeck state
    SelectCardFromDeck(usize),

    /// Buy a card from the shop (0-based index)
    /// Only valid when in shop
    BuyCard(usize),

    /// Perform a shop action (card removal, etc.)
    /// Only valid when in shop
    ShopAction(ShopAction),
}

/// Rest site actions that can be chosen by the player
#[derive(Debug, Clone, PartialEq)]
pub enum ShopAction {
    /// Buy a card from the shop (0-based index)
    BuyCard(usize),
    EnterCardRemoval,
}

/// Rest site actions that can be chosen by the player
#[derive(Debug, Clone, PartialEq)]
pub enum RestSiteAction {
    /// Rest and heal 30% of max HP (minimum 15)
    Rest,
    /// Upgrade a card from the deck (remove it, add a better version)
    Upgrade,
}

/// Result of ending a run
#[derive(Debug, Clone, PartialEq)]
pub enum RunResult {
    /// Player died in battle
    Death,
    /// Player completed all floors and won
    Victory,
    /// Player chose to abandon the run
    Abandon,
}