use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

pub fn flex() -> Card {
    Card::new(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength(2),        // Gain 2 Strength immediately
        Effect::LoseStrengthAtEndOfTurn(2), // Lose 2 Strength at end of turn
    ], false, true)
}

pub fn flex_upgraded() -> Card {
    Card::new(CardEnum::Flex, 0, CardType::Skill, vec![
        Effect::GainStrength(3),        // Gain 3 Strength immediately (+1)
        Effect::LoseStrengthAtEndOfTurn(3), // Lose 3 Strength at end of turn
    ], true, true)
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
}