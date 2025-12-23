use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Nunchaku - Every time you play 10 Attacks, gain 1 Energy
pub struct NunchakuRelic {
    attack_count: u8,
    owner: Entity,
}

impl NunchakuRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            attack_count: 0,
            owner,
        }
    }
}

impl EventListener for NunchakuRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.attack_count = 0;
                vec![]
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner => {
                use crate::game::card_type::CardType;
                if *card_type == CardType::Attack {
                    self.attack_count += 1;
                    if self.attack_count >= 10 {
                        self.attack_count = 0;
                        vec![BattleEffect::GainEnergy { amount: 1 }]
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
    use crate::game::card_type::CardType;

    #[test]
    fn test_nunchaku_creation() {
        let nunchaku = NunchakuRelic::new(Entity::Player);
        assert_eq!(nunchaku.attack_count, 0);
    }

    #[test]
    fn test_nunchaku_resets_on_combat_start() {
        let mut nunchaku = NunchakuRelic::new(Entity::Player);
        nunchaku.attack_count = 5;

        let _ = nunchaku.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        assert_eq!(nunchaku.attack_count, 0);
    }

    #[test]
    fn test_nunchaku_counts_attacks() {
        let mut nunchaku = NunchakuRelic::new(Entity::Player);

        for i in 1..=9 {
            let effects = nunchaku.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            assert_eq!(effects.len(), 0);
            assert_eq!(nunchaku.attack_count, i as u8);
        }
    }

    #[test]
    fn test_nunchaku_gives_energy_every_10_attacks() {
        let mut nunchaku = NunchakuRelic::new(Entity::Player);

        // Play 10 attacks
        for _ in 0..9 {
            let effects = nunchaku.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            assert_eq!(effects.len(), 0);
        }

        // 10th attack should trigger
        let effects = nunchaku.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::GainEnergy { amount: 1 }));
        assert_eq!(nunchaku.attack_count, 0); // Reset after triggering
    }

    #[test]
    fn test_nunchaku_only_counts_attacks() {
        let mut nunchaku = NunchakuRelic::new(Entity::Player);

        // Play a skill (should not count)
        let effects1 = nunchaku.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects1.len(), 0);
        assert_eq!(nunchaku.attack_count, 0);

        // Play an attack (should count)
        let effects2 = nunchaku.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects2.len(), 0);
        assert_eq!(nunchaku.attack_count, 1);
    }

    #[test]
    fn test_nunchaku_no_trigger_for_enemy_cards() {
        let mut nunchaku = NunchakuRelic::new(Entity::Player);

        let effects = nunchaku.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Attack,
        });

        assert_eq!(effects.len(), 0);
        assert_eq!(nunchaku.attack_count, 0);
    }
}
