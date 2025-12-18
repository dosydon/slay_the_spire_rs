//! Global game actions that can be performed throughout the entire game

use bevy::prelude::Event;
use crate::battle::action::Action as BattleAction;

/// High-level game actions that encompass the entire game flow
#[derive(Debug, Clone, PartialEq, Event)]
pub enum GameAction {
    /// Execute a battle action (PlayCard, EndTurn, etc.)
    /// Only valid when in battle
    Battle(BattleAction),

    /// Choose a path on the map (0-based index)
    /// Determines what type of encounter comes next
    ChoosePath(usize),

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
    SelectCardToUpgrade(usize),

    /// Buy a card from the shop (0-based index)
    /// Only valid when in shop
    BuyCard(usize),

    /// Leave the shop
    /// Only valid when in shop
    LeaveShop,
}

/// Rest site actions that can be chosen by the player
#[derive(Debug, Clone, PartialEq)]
pub enum RestSiteAction {
    /// Rest and heal 30% of max HP (minimum 15)
    Rest,
    /// Upgrade a card from the deck (remove it, add a better version)
    Upgrade,
    /// Remove a card from the deck
    Remove,
    /// Obtain gold (15 gold)
    ObtainGold,
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