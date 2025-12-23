use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::Effect;
use crate::battle::target::Entity;
use crate::game::card_type::CardType;

/// Letter Opener - Every time you play 3 Skills in a single turn, deal 5 damage to ALL enemies
pub struct LetterOpenerRelic {
    skills_this_turn: u32,
    owner: Entity,
}

impl LetterOpenerRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            skills_this_turn: 0,
            owner,
        }
    }
}

impl EventListener for LetterOpenerRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.skills_this_turn = 0;
                vec![]
            }
            BattleEvent::StartOfPlayerTurn => {
                // Reset skill counter at start of turn
                self.skills_this_turn = 0;
                vec![]
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner => {
                if *card_type == CardType::Skill {
                    self.skills_this_turn += 1;
                    // Check if we've played 3, 6, 9, etc. skills this turn
                    if self.skills_this_turn % 3 == 0 && self.skills_this_turn > 0 {
                        vec![Effect::AttackAllEnemies {
                            amount: 5,
                            num_attacks: 1,
                        }]
                    } else {
                        vec![]
                    }
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

    #[test]
    fn test_letter_opener_creation() {
        let opener = LetterOpenerRelic::new(Entity::Player);
        assert_eq!(opener.skills_this_turn, 0);
    }

    #[test]
    fn test_letter_opener_deals_damage_every_3_skills() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        // Combat start
        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn start
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 3 skills
        let effects1 = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects1.len(), 0);

        let effects2 = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects2.len(), 0);

        let effects3 = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects3.len(), 1);
        assert!(matches!(effects3[0], Effect::AttackAllEnemies { amount: 5, num_attacks: 1 }));
    }

    #[test]
    fn test_letter_opener_resets_on_turn_start() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        // Combat start and turn 1
        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 2 skills
        let _ = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        let _ = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(opener.skills_this_turn, 2);

        // Turn 2 starts - should reset
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(opener.skills_this_turn, 0);
    }

    #[test]
    fn test_letter_opener_only_counts_skills() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play attacks and powers - shouldn't count
        let effects1 = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects1.len(), 0);
        assert_eq!(opener.skills_this_turn, 0);

        let effects2 = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Power,
        });
        assert_eq!(effects2.len(), 0);
        assert_eq!(opener.skills_this_turn, 0);
    }

    #[test]
    fn test_letter_opener_triggers_multiple_times_per_turn() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 6 skills - should trigger twice
        let mut trigger_count = 0;
        for _ in 0..6 {
            let effects = opener.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Skill,
            });
            if effects.len() > 0 {
                trigger_count += 1;
            }
        }

        assert_eq!(trigger_count, 2);
    }

    #[test]
    fn test_letter_opener_no_trigger_for_enemy_cards() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Enemy plays skill - shouldn't count
        let effects = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Skill,
        });

        assert_eq!(effects.len(), 0);
        assert_eq!(opener.skills_this_turn, 0);
    }

    #[test]
    fn test_letter_opener_resets_on_combat_start() {
        let mut opener = LetterOpenerRelic::new(Entity::Player);

        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = opener.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play some skills
        let _ = opener.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(opener.skills_this_turn, 1);

        // New combat resets
        let _ = opener.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(opener.skills_this_turn, 0);
    }
}
