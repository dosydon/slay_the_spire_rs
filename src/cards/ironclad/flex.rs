use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

pub fn flex() -> Card {
    Card::new_with_condition(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength { amount: 2 },        // Gain 2 Strength immediately
        Effect::LoseStrengthAtEndOfTurn(2), // Lose 2 Strength at end of turn
    ], false, Condition::True, Rarity::Common)
}

pub fn flex_upgraded() -> Card {
    Card::new_with_condition(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength { amount: 3 },        // Gain 3 Strength immediately (+1)
        Effect::LoseStrengthAtEndOfTurn(3), // Lose 3 Strength at end of turn
    ], true, Condition::True, Rarity::Common)
}

// LoseStrengthListener implementation for Flex card
#[derive(Debug)]
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
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                self.is_active = false; // Only trigger once
                vec![Effect::LoseStrengthSelf(self.amount_to_lose)]
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
}