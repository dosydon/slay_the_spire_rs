use crate::battle::{battle_action::BattleAction, Battle};
use crate::battle::target::Entity;
use crate::game::{card::Card, card_type::CardType, effect::BattleEffect};

impl Battle {
    pub fn list_available_actions(&self) -> Vec<BattleAction> {
        let mut available_actions = Vec::new();

        // Battle is over - no actions available
        if self.is_battle_over() {
            return available_actions;
        }

        // Check each card in hand
        let hand = self.cards.get_hand();
        for (card_index, card) in hand.iter().enumerate() {
            // Check if card is playable, player has enough energy, and card is not an Attack while Entangled
            let is_attack_while_entangled = self.player.battle_info.is_entangled()
                && card.get_card_type() == CardType::Attack;

            if card.is_playable() && self.player.get_energy() >= card.get_cost() && !is_attack_while_entangled {
                // Determine valid targets for this card based on its type and effects
                let valid_targets = self.get_valid_targets_for_card(card);

                // Add PlayCard action for each valid target
                for target in valid_targets {
                    available_actions.push(BattleAction::PlayCard(card_index, target));
                }
            }
        }

        // Add UsePotion actions for each filled potion slot
        let potions = self.get_potions();
        for (slot_index, potion) in potions.get_all_potions() {
            let (default_target, _effects) = potion.get_effects();

            // If potion has a default target, add action with None (will use default)
            if default_target.is_some() {
                available_actions.push(BattleAction::UsePotion(slot_index, None));
            } else {
                // Potion requires target selection - add action for each valid enemy
                for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                    if enemy.battle_info.is_alive() {
                        available_actions.push(BattleAction::UsePotion(slot_index, Some(Entity::Enemy(enemy_index))));
                    }
                }
            }
        }

        // EndTurn is always available when battle is not over
        available_actions.push(BattleAction::EndTurn);

        available_actions
    }

    /// Get valid targets for a specific card based on its effects
    pub(in crate::battle) fn get_valid_targets_for_card(&self, card: &Card) -> Vec<Entity> {
        let mut valid_targets = Vec::new();

        // Check if any effect attacks all enemies (doesn't need specific targeting)
        let attacks_all_enemies = card.get_effects().iter().any(|effect| {
            matches!(effect, BattleEffect::AttackAllEnemies { .. })
        });

        // Check if any effect targets specific enemies
        let targets_specific_enemies = card.get_effects().iter().any(|effect| {
            matches!(effect,
                BattleEffect::AttackToTarget { .. } |
                BattleEffect::ApplyVulnerable { .. } |
                BattleEffect::ApplyWeak { .. }
            )
        });

        // Check if any effect targets self/player
        let targets_self = card.get_effects().iter().any(|effect| {
            matches!(effect,
                BattleEffect::GainDefense { amount: _ } |
                BattleEffect::GainStrength { amount: _ }
            )
        });

        // AttackAllEnemies cards use Entity::None as target (no specific targeting needed)
        if attacks_all_enemies {
            valid_targets.push(Entity::None);
        }

        // Add valid enemy targets for specific targeting
        if targets_specific_enemies {
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                if enemy.battle_info.is_alive() {
                    valid_targets.push(Entity::Enemy(enemy_index));
                }
            }
        }

        // Add player target
        if targets_self {
            valid_targets.push(Entity::Player);
        }

        // If no specific targeting logic applies, default to allowing enemy targets only
        // This handles cards with unknown effect types - default to assuming they target enemies
        if valid_targets.is_empty() {
            // Add all alive enemies as potential targets
            for (enemy_index, enemy) in self.enemies.iter().enumerate() {
                if enemy.battle_info.is_alive() {
                    valid_targets.push(Entity::Enemy(enemy_index));
                }
            }
        }

        valid_targets
    }

    /// Check if a target is valid for the current battle state
    pub(in crate::battle) fn is_valid_target(&self, target: &Entity) -> bool {
        match target {
            Entity::Enemy(idx) => *idx < self.enemies.len(),
            Entity::Player => true,  // Player is always a valid target
            Entity::None => true,    // None is valid for AttackAllEnemies cards
        }
    }
}
