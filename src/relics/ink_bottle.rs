use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;

/// Ink Bottle - Every time you play 10 cards, draw 1 card
pub struct InkBottleRelic {
    card_count: u32,
    owner: Entity,
}

impl InkBottleRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            card_count: 0,
            owner,
        }
    }
}

impl EventListener for InkBottleRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.card_count = 0;
                vec![]
            }
            BattleEvent::CardPlayed { source, .. } if *source == self.owner => {
                self.card_count += 1;
                if self.card_count % 10 == 0 && self.card_count > 0 {
                    vec![Effect::DrawCard { count: 1 }]
                } else {
                    vec![]
                }
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card_type::CardType;

    #[test]
    fn test_ink_bottle_creation() {
        let bottle = InkBottleRelic::new(Entity::Player);
        assert_eq!(bottle.card_count, 0);
    }

    #[test]
    fn test_ink_bottle_draws_every_10_cards() {
        let mut bottle = InkBottleRelic::new(Entity::Player);

        // Reset on combat start
        let _ = bottle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Play 10 cards
        let mut trigger_count = 0;
        for i in 1..=10 {
            let effects = bottle.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            if i == 10 {
                assert_eq!(effects.len(), 1);
                trigger_count += 1;
            } else {
                assert_eq!(effects.len(), 0);
            }
        }

        assert_eq!(trigger_count, 1);
        assert_eq!(bottle.card_count, 10);
    }

    #[test]
    fn test_ink_bottle_resets_on_combat_start() {
        let mut bottle = InkBottleRelic::new(Entity::Player);

        // Play 5 cards
        for _ in 0..5 {
            let _ = bottle.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Skill,
            });
        }
        assert_eq!(bottle.card_count, 5);

        // Combat start resets
        let _ = bottle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(bottle.card_count, 0);
    }

    #[test]
    fn test_ink_bottle_counts_all_cards() {
        let mut bottle = InkBottleRelic::new(Entity::Player);

        let _ = bottle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Play mixed cards
        let _ = bottle.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(bottle.card_count, 1);

        let _ = bottle.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(bottle.card_count, 2);

        let _ = bottle.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Power,
        });
        assert_eq!(bottle.card_count, 3);
    }

    #[test]
    fn test_ink_bottle_no_trigger_for_enemy_cards() {
        let mut bottle = InkBottleRelic::new(Entity::Player);

        // Enemy plays card - shouldn't count
        let effects = bottle.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Attack,
        });

        assert_eq!(effects.len(), 0);
        assert_eq!(bottle.card_count, 0);
    }

    #[test]
    fn test_ink_bottle_triggers_multiple_times() {
        let mut bottle = InkBottleRelic::new(Entity::Player);

        let _ = bottle.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Play 20 cards - should trigger twice
        let mut trigger_count = 0;
        for i in 1..=20 {
            let effects = bottle.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            if effects.len() > 0 {
                trigger_count += 1;
            }
        }

        assert_eq!(trigger_count, 2);
        assert_eq!(bottle.card_count, 20);
    }
}
