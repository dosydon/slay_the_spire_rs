use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;
use crate::game::card_type::CardType;

/// Art of War - If you do not play Attacks during your turn, gain 1 Energy next turn
pub struct ArtOfWarRelic {
    played_attack_this_turn: bool,
    should_grant_energy: bool,
    owner: Entity,
}

impl ArtOfWarRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            played_attack_this_turn: false,
            should_grant_energy: false,
            owner,
        }
    }
}

impl EventListener for ArtOfWarRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.played_attack_this_turn = false;
                self.should_grant_energy = false;
                vec![]
            }
            BattleEvent::StartOfPlayerTurn => {
                // Grant energy if we didn't play attacks last turn
                if self.should_grant_energy {
                    self.should_grant_energy = false;
                    vec![BattleEffect::GainEnergy { amount: 1 }]
                } else {
                    vec![]
                }
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner => {
                if *card_type == CardType::Attack {
                    self.played_attack_this_turn = true;
                }
                vec![]
            }
            BattleEvent::EndOfTurn { entity } if *entity == self.owner => {
                // If we didn't play attacks this turn, grant energy next turn
                if !self.played_attack_this_turn {
                    self.should_grant_energy = true;
                }
                // Reset attack tracking for next turn
                self.played_attack_this_turn = false;
                vec![]
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
    fn test_art_of_war_creation() {
        let art = ArtOfWarRelic::new(Entity::Player);
        assert!(!art.played_attack_this_turn);
        assert!(!art.should_grant_energy);
    }

    #[test]
    fn test_art_of_war_gains_energy_without_attacks() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Combat start
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn 1 starts - no energy (no previous turn to check)
        let effects1 = art.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 0);

        // End turn without playing attacks
        let _ = art.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Player,
        });

        // Turn 2 starts - should gain energy
        let effects2 = art.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 1);
        assert!(matches!(effects2[0], BattleEffect::GainEnergy { amount: 1 }));
    }

    #[test]
    fn test_art_of_war_no_energy_with_attacks() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Combat start
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn 1 starts
        let _ = art.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play an attack
        let _ = art.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert!(art.played_attack_this_turn);

        // End turn
        let _ = art.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Player,
        });

        // Turn 2 starts - should NOT gain energy
        let effects = art.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_art_of_war_resets_on_combat_start() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Play an attack
        let _ = art.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert!(art.played_attack_this_turn);

        // Combat start resets
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert!(!art.played_attack_this_turn);
        assert!(!art.should_grant_energy);
    }

    #[test]
    fn test_art_of_war_only_counts_attacks() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Combat start
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn 1 starts
        let _ = art.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play skills and powers (not attacks)
        let _ = art.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        let _ = art.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Power,
        });

        // Should not have played attack
        assert!(!art.played_attack_this_turn);
    }

    #[test]
    fn test_art_of_war_no_trigger_for_enemy_cards() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Combat start
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Enemy plays attack - shouldn't count
        let _ = art.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Attack,
        });

        assert!(!art.played_attack_this_turn);
    }

    #[test]
    fn test_art_of_war_continuous_turns() {
        let mut art = ArtOfWarRelic::new(Entity::Player);

        // Combat start
        let _ = art.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn 1: No attacks
        let _ = art.on_event(&BattleEvent::StartOfPlayerTurn);
        let _ = art.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Player,
        });

        // Turn 2: Should gain energy
        let effects1 = art.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects1.len(), 1);

        // Play no attacks again
        let _ = art.on_event(&BattleEvent::EndOfTurn {
            entity: Entity::Player,
        });

        // Turn 3: Should gain energy again
        let effects2 = art.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(effects2.len(), 1);
    }
}
