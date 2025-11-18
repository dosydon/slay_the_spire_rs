use crate::{battle::target::Entity, game::effect::Effect};

#[derive(Debug, Clone, PartialEq)]
pub enum BattleEvent {
    DamageTaken {
        target: Entity,
        amount: u32,
        source: Entity
    },
    HpLostFromCard {
        target: Entity,
        amount: u32,
    },
    SkillCardPlayed {
        source: Entity,
    },
    CardPlayed {
        source: Entity,
        card_type: crate::game::card_type::CardType,
    },
    CardExhausted {
        source: Entity,
    },
    TurnStart {
        entity: Entity,
    },
    EndOfTurn {
        entity: Entity,
    },
    CombatVictory {
        player: Entity,
    },
    CombatStart {
        player: Entity,
    },
}

pub trait EventListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect>;
    fn is_active(&self) -> bool;
    fn get_owner(&self) -> Entity;
}