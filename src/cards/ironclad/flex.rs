use serde::{Serialize, Deserialize};
use crate::game::{card::Card, effect::{BattleEffect, Condition}, card_type::CardType, card_enum::CardEnum, card::{Rarity, CardClass}};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

pub fn flex() -> Card {
    Card::new(CardEnum::Flex, 0, CardClass::IronClad(Rarity::Common, CardType::Skill), vec![
        BattleEffect::GainStrength { amount: 2 },        // Gain 2 Strength immediately
        BattleEffect::LoseStrengthAtEndOfTurn(2), // Lose 2 Strength at end of turn
    ])
        .set_play_condition(Condition::True)
}

pub fn flex_upgraded() -> Card {
    Card::new(CardEnum::Flex, 0, CardClass::IronClad(Rarity::Common, CardType::Skill), vec![
        BattleEffect::GainStrength { amount: 3 },        // Gain 3 Strength immediately (+1)
        BattleEffect::LoseStrengthAtEndOfTurn(3), // Lose 3 Strength at end of turn
    ])
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}

// LoseStrengthListener implementation for Flex card
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoseStrengthListener {
    amount_to_lose: u32,
    owner: Entity,
    is_active: bool,
}

impl LoseStrengthListener {
    pub(crate) fn new(owner: Entity, amount_to_lose: u32) -> Self {
        LoseStrengthListener {
            amount_to_lose,
            owner,
            is_active: true,
        }
    }
}

impl EventListener for LoseStrengthListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                self.is_active = false; // Only trigger once
                vec![BattleEffect::LoseStrengthSelf(self.amount_to_lose)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}