use super::Battle;
use crate::battle::{target::Entity, BattleError, events::BattleEvent};
use crate::game::{effect::BaseEffect, card_type::CardType};

impl Battle {
    /// Play a card from hand targeting a specific entity
    pub(crate) fn play_card(&mut self, idx: usize, target: Entity) -> Result<(), BattleError> {
        if idx >= self.cards.hand_size() {
            return Err(BattleError::CardNotInHand);
        }

        let hand = self.cards.get_hand();
        let card = &hand[idx];

        // Check if card is playable
        if !card.is_playable() {
            return Err(BattleError::CardNotPlayable);
        }

        if !self.player.spend_energy(card.get_cost()) {
            return Err(BattleError::NotEnoughEnergy);
        }

        let card_effects = card.get_effects().clone();
        let is_skill_card = card.get_card_type() == &CardType::Skill;
        let is_power_card = card.get_card_type() == &CardType::Power;
        let has_exhaust = card_effects.contains(&crate::game::effect::Effect::Exhaust);

        // Emit SkillCardPlayed event if this is a Skill card
        if is_skill_card {
            let skill_event = BattleEvent::SkillCardPlayed {
                source: Entity::Player,
            };
            self.emit_event(skill_event);
        }

        // Handle different card types
        let result = if is_power_card {
            // Power cards are removed from hand but NOT added to discard pile (they stay in play)
            if let Some(played_card) = self.cards.remove_card_from_hand(idx) {
                // Add to powers collection
                self.powers.push(played_card.clone());

                // Queue all effects
                for effect in card_effects {
                    self.queue_effect(BaseEffect::from_effect(effect, Entity::Player, target));
                }
                Ok(())
            } else {
                Err(BattleError::CardNotInHand)
            }
        } else if has_exhaust {
            // Cards with Exhaust stay in hand until the Exhaust effect is processed
            // Queue all effects, with special handling for Exhaust to include the hand index
            for effect in card_effects {
                if effect == crate::game::effect::Effect::Exhaust {
                    // Create Exhaust effect with hand index
                    self.queue_effect(BaseEffect::Exhaust {
                        hand_index: idx,
                    });
                } else {
                    self.queue_effect(BaseEffect::from_effect(effect, Entity::Player, target));
                }
            }
            Ok(())
        } else {
            // Regular cards (Attack, Skill, Status without Exhaust) go to discard pile
            if let Some(_played_card) = self.cards.play_card_from_hand(idx) {
                // Queue all effects
                for effect in card_effects {
                    self.queue_effect(BaseEffect::from_effect(effect, Entity::Player, target));
                }
                Ok(())
            } else {
                Err(BattleError::CardNotInHand)
            }
        };

        // Process all queued effects
        self.process_effect_queue();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_exhaust_card_functionality() {
        use crate::cards::ironclad::{strike::strike, defend::defend};

        let mut deck_cards = vec![strike(), defend(), strike(), defend(), strike()];
        // Add a Slimed card to the deck
        deck_cards.push(crate::cards::status::slimed::slimed());
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        // Draw the hand
        battle.cards.draw_n(5);

        // Find the Slimed card in hand (if any) or add one
        let mut slimed_index = None;
        let hand = battle.cards.get_hand();
        for (i, card) in hand.iter().enumerate() {
            if card.get_name() == "Slimed" {
                slimed_index = Some(i);
                break;
            }
        }

        // If no Slimed card in hand, add one manually for testing
        if slimed_index.is_none() {
            battle.cards.add_card_to_hand(crate::cards::status::slimed::slimed());
            slimed_index = Some(battle.cards.hand_size() - 1);
        }

        let slimed_idx = slimed_index.unwrap();
        let initial_hand_size = battle.cards.hand_size();
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_exhausted_size = battle.cards.exhausted_size();
        let initial_energy = battle.player.get_energy();

        // Play the Slimed card
        let _ = battle.play_card(slimed_idx, Entity::Player);

        // Verify the effects:
        // 1. Card should be removed from hand
        assert_eq!(battle.cards.hand_size(), initial_hand_size - 1);
        // 2. Card should NOT go to discard pile (it's exhausted)
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size);
        // 3. Card should go to exhausted pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhausted_size + 1);
        // 4. Energy should be reduced by 1 (Slimed costs 1)
        assert_eq!(battle.player.get_energy(), initial_energy - 1);

        // Check that the exhausted card is Slimed
        let exhausted_cards = battle.cards.get_exhausted();
        assert_eq!(exhausted_cards.last().unwrap().get_name(), "Slimed");
    }
}
