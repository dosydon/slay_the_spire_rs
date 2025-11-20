use super::Battle;
use crate::game::effect::BaseEffect;
use crate::battle::{target::Entity, events::BattleEvent};
use crate::enemies::gremlin_nob::EnrageListener;
use rand::prelude::IndexedRandom;

impl Battle {
    /// Apply a specific effect with its target
    pub(crate) fn eval_base_effect(&mut self, effect: &BaseEffect) {
        match effect {
            BaseEffect::AttackToTarget { source, target, amount, num_attacks, strength_multiplier } => {
                for _ in 0..*num_attacks {
                    let incoming_damage = self.calculate_incoming_damage_with_multiplier(*source, *target, *amount, *strength_multiplier);
                    self.apply_damage(*target, incoming_damage);
                }
            },
            BaseEffect::AttackToTargetWithBlock { source: _, target } => {
                // Deal damage equal to player's current Block
                let damage_amount = self.player.get_block();
                if damage_amount > 0 {
                    self.apply_damage(*target, damage_amount);
                }
            },
            BaseEffect::AttackToTargetWithScaling { source, target, base_damage, scaling } => {
                // Calculate damage with current rampage scaling
                let total_damage = base_damage + self.player.battle_info.get_rampage_damage();
                let incoming_damage = self.calculate_incoming_damage_with_multiplier(*source, *target, total_damage, 1);
                self.apply_damage(*target, incoming_damage);

                // Increase rampage scaling for next use
                self.player.battle_info.increase_rampage_damage(*scaling);
            },
            BaseEffect::AttackAllEnemies { source, amount, num_attacks } => {
                for _ in 0..*num_attacks {
                    for enemy_idx in 0..self.enemies.len() {
                        if self.enemies[enemy_idx].battle_info.is_alive() {
                            let target = Entity::Enemy(enemy_idx);
                            let incoming_damage = self.calculate_incoming_damage(*source, target, *amount);
                            self.apply_damage(target, incoming_damage);
                        }
                    }
                }
            },
            BaseEffect::GainDefense { source, amount } => {
                // Defense effects apply to the source entity
                self.apply_block(*source, *amount);
            },
            BaseEffect::ApplyVulnerable { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_vulnerable(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_vulnerable(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::ApplyVulnerableAll { duration } => {
                // Apply vulnerable to all enemies
                for enemy_idx in 0..self.enemies.len() {
                    if self.enemies[enemy_idx].battle_info.is_alive() {
                        self.enemies[enemy_idx].battle_info.apply_vulnerable(*duration);
                    }
                }
            },
            BaseEffect::ApplyWeak { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_weak(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_weak(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::GainStrength { source, amount } => {
                match source {
                    Entity::Player => self.player.battle_info.gain_strength(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.gain_strength(*amount);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::DoubleStrength { source } => {
                match source {
                    Entity::Player => {
                        let current_strength = self.player.battle_info.get_strength();
                        self.player.battle_info.set_strength(current_strength * 2);
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            let current_strength = self.enemies[*idx].battle_info.get_strength();
                            self.enemies[*idx].battle_info.set_strength(current_strength * 2);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::LoseStrengthSelf { source, amount } => {
                match source {
                    Entity::Player => {
                        // Reduce player strength, but not below 0
                        let current_strength = self.player.battle_info.get_strength();
                        let actual_loss = std::cmp::min(*amount, current_strength);
                        self.player.battle_info.strength = current_strength - actual_loss;
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            // Reduce enemy strength, but not below 0
                            let current_strength = self.enemies[*idx].battle_info.get_strength();
                            let actual_loss = std::cmp::min(*amount, current_strength);
                            self.enemies[*idx].battle_info.strength = current_strength - actual_loss;
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::LoseStrengthTarget { target, amount } => {
                match target {
                    Entity::Player => {
                        // Reduce player strength, but not below 0
                        let current_strength = self.player.battle_info.get_strength();
                        let actual_loss = std::cmp::min(*amount, current_strength);
                        self.player.battle_info.strength = current_strength - actual_loss;
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            // Reduce enemy strength, but not below 0
                            let current_strength = self.enemies[*idx].battle_info.get_strength();
                            let actual_loss = std::cmp::min(*amount, current_strength);
                            self.enemies[*idx].battle_info.strength = current_strength - actual_loss;
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::GainRitual { source, amount } => {
                match source {
                    Entity::Player => self.player.battle_info.gain_ritual(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.gain_ritual(*amount);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::ApplyFrail { target, duration } => {
                match target {
                    Entity::Player => {
                        self.player.battle_info.apply_frail(*duration);
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_frail(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::AddSlimed { target, count } => {
                match target {
                    Entity::Player => {
                        for _ in 0..*count {
                            let slimed_card = crate::cards::status::slimed::slimed();
                            self.cards.add_card_to_discard(slimed_card);
                        }
                    },
                    Entity::Enemy(_) => {
                        // Enemies don't receive slimed cards
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::Exhaust { hand_index } => {
                // Remove card from hand and add to exhausted pile
                if let Some(_card) = self.cards.exhaust_card_from_hand(*hand_index) {
                    // Emit CardExhausted event
                    let exhaust_event = BattleEvent::CardExhausted {
                        source: Entity::Player,
                    };
                    self.emit_event(exhaust_event);
                }
            },
            BaseEffect::LoseStrengthAtEndOfTurn { source, amount } => {
                // Create a LoseStrengthListener to handle strength loss at end of turn
                let lose_listener = crate::cards::ironclad::flex::LoseStrengthListener::new(*source, *amount);
                self.add_listener(Box::new(lose_listener));
            },
            BaseEffect::ActivateEnrage { source, amount } => {
                // Add EnrageListener for the specified enemy
                if let Entity::Enemy(_enemy_idx) = source {
                    let enrage_listener = EnrageListener::new(*source, *amount);
                    self.add_listener(Box::new(enrage_listener));
                }
            },
            BaseEffect::ActivateEmbrace { source } => {
                // Add EmbraceListener for the player
                if let Entity::Player = source {
                    let embrace_listener = crate::cards::ironclad::embrace::EmbraceListener::new(*source);
                    self.add_listener(Box::new(embrace_listener));
                }
            },
            BaseEffect::ActivateFeelNoPain { source, block_per_exhaust } => {
                // Add FeelNoPainListener for the player
                if let Entity::Player = source {
                    let feel_no_pain_listener = crate::cards::ironclad::feel_no_pain::FeelNoPainListener::new(*source, *block_per_exhaust);
                    self.add_listener(Box::new(feel_no_pain_listener));
                }
            },
            BaseEffect::ActivateBrutality { source } => {
                // Add BrutalityListener for the player
                if let Entity::Player = source {
                    let brutality_listener = crate::cards::ironclad::brutality::BrutalityListener::new(*source);
                    self.add_listener(Box::new(brutality_listener));
                }
            },
            BaseEffect::ActivateCorruption { source } => {
                // Add CorruptionListener for the player
                if let Entity::Player = source {
                    let corruption_listener = crate::cards::ironclad::corruption::CorruptionListener::new(*source);
                    self.add_listener(Box::new(corruption_listener));
                }
            },
            BaseEffect::ActivateMetallicize { source, amount } => {
                // Add MetallicizeListener for the player
                if let Entity::Player = source {
                    let metallicize_listener = crate::cards::ironclad::metallicize::MetallicizeListener::new(*source, *amount);
                    self.add_listener(Box::new(metallicize_listener));
                }
            },
            BaseEffect::ActivateFlameBarrier { source, damage } => {
                // Add FlameBarrierListener for the player
                if let Entity::Player = source {
                    let flame_barrier_listener = crate::cards::ironclad::flame_barrier::FlameBarrierListener::new(*source, *damage);
                    self.add_listener(Box::new(flame_barrier_listener));
                }
            },
            BaseEffect::ActivateBurn { source, damage } => {
                // Add BurnListener for the player
                if let Entity::Player = source {
                    let burn_listener = crate::cards::status::burn::BurnListener::new(*source, *damage);
                    self.add_listener(Box::new(burn_listener));
                }
            },
            BaseEffect::ActivateDemonForm { source, strength_per_turn } => {
                // Add DemonFormListener for the player
                if let Entity::Player = source {
                    let demon_form_listener = crate::cards::ironclad::demon_form::DemonFormListener::new(*source, *strength_per_turn);
                    self.add_listener(Box::new(demon_form_listener));
                }
            },
            BaseEffect::ActivateRage { source, block_per_attack } => {
                // Add RageListener for the player
                if let Entity::Player = source {
                    let rage_listener = crate::cards::ironclad::rage::RageListener::new(*source, *block_per_attack);
                    self.add_listener(Box::new(rage_listener));
                }
            },
            BaseEffect::AddRandomAttackToHand { source } => {
                // Add a random Ironclad Attack card to hand
                if let Entity::Player = source {
                    let ironclad_attacks = vec![
                        crate::cards::ironclad::strike::strike(),
                        crate::cards::ironclad::bash::bash(),
                        crate::cards::ironclad::cleave::cleave(),
                        crate::cards::ironclad::clothesline::clothesline(),
                        crate::cards::ironclad::heavy_blade::heavy_blade(),
                        crate::cards::ironclad::iron_wave::iron_wave(),
                        crate::cards::ironclad::perfected_strike::perfected_strike(),
                        crate::cards::ironclad::pommel_strike::pommel_strike(),
                        crate::cards::ironclad::thunderclap::thunderclap(),
                        crate::cards::ironclad::twin_strike::twin_strike(),
                        crate::cards::ironclad::wild_strike::wild_strike(),
                        crate::cards::ironclad::body_slam::body_slam(),
                        crate::cards::ironclad::carnage::carnage(),
                        crate::cards::ironclad::clash::clash(),
                        crate::cards::ironclad::headbutt::headbutt(),
                        crate::cards::ironclad::hemokinesis::hemokinesis(),
                        crate::cards::ironclad::sword_boomerang::sword_boomerang(),
                    ];

                    if let Some(random_attack) = ironclad_attacks.choose(&mut rand::rng()) {
                        self.cards.add_card_to_hand(random_attack.clone());
                    }
                }
            },
            BaseEffect::ActivateEvolve { source: _ } => {
                // Evolve activation - currently just draws 1 card immediately
                // In full implementation, would add a listener for Status card draws
                for _ in 0..1 {
                    self.cards.draw_card();
                }
            },
            BaseEffect::AddCardToDrawPile { source: _, card } => {
                // Add a specific card to the draw pile
                let card = match card {
                    crate::game::card_enum::CardEnum::Wound => crate::cards::status::wound::wound(),
                    crate::game::card_enum::CardEnum::Slimed => crate::cards::status::slimed::slimed(),
                    _ => return, // Unsupported card type
                };
                self.cards.add_card_to_deck(card);
            },
            BaseEffect::DrawCard { source: _, count } => {
                // Draw cards for the player
                self.cards.draw_n(*count as usize);
            },
            BaseEffect::Heal { target, amount } => {
                // Heal the target entity
                match target {
                    Entity::Player => self.player.battle_info.heal(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.heal(*amount);
                        }
                    },
                    Entity::None => {} // No target, no healing
                }
            },
            BaseEffect::GainPlatedArmor { source, amount } => {
                // Add plated armor to the source entity
                match source {
                    Entity::Player => {
                        // TODO: Implement plated armor system
                        // For now, treat as regular block gain
                        self.apply_block(*source, *amount);
                    },
                    Entity::Enemy(_) => {
                        // Enemies typically don't get plated armor
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::DoubleBlock { source } => {
                // Double the current block of the source entity
                match source {
                    Entity::Player => {
                        let current_block = self.player.get_block();
                        let additional_block = current_block; // Add the same amount again
                        self.apply_block(*source, additional_block);
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            let current_block = self.enemies[*idx].battle_info.get_block();
                            let additional_block = current_block;
                            self.apply_block(*source, additional_block);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::ActivateCombust { source, amount } => {
                // Add CombustListener for the player
                if let Entity::Player = source {
                    let combust_listener = crate::cards::ironclad::combust::CombustListener::new(*source, *amount);
                    self.add_listener(Box::new(combust_listener));
                }
            },
            BaseEffect::ApplyDamageReduction { target, percentage } => {
                // Apply damage reduction to the target entity
                match target {
                    Entity::Player => {
                        // TODO: Implement damage reduction status effect system
                        // For now, this effect has no implementation
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            // TODO: Implement damage reduction status effect system
                            // For now, this effect has no implementation
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::LoseHp { target, amount } => {
                // Direct HP loss (ignores block)
                match target {
                    Entity::Player => {
                        let current_hp = self.player.battle_info.get_current_hp();
                        let new_hp = current_hp.saturating_sub(*amount);
                        self.player.battle_info.set_current_hp(new_hp);

                        // Emit HP loss from card event for Rupture and other listeners
                        if *amount > 0 {
                            let hp_loss_event = BattleEvent::HpLostFromCard {
                                target: *target,
                                amount: *amount,
                            };
                            self.emit_event(hp_loss_event);
                        }
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            let current_hp = self.enemies[*idx].battle_info.get_current_hp();
                            let new_hp = current_hp.saturating_sub(*amount);
                            self.enemies[*idx].battle_info.set_current_hp(new_hp);

                            // Emit HP loss from card event for listeners
                            if *amount > 0 {
                                let hp_loss_event = BattleEvent::HpLostFromCard {
                                    target: *target,
                                    amount: *amount,
                                };
                                self.emit_event(hp_loss_event);
                            }
                        }
                    },
                    Entity::None => {} // No target, no HP loss
                }
            },
            BaseEffect::GainEnergy { source: _, amount } => {
                // Gain energy
                self.player.battle_info.gain_energy(*amount);
            },
            BaseEffect::ApplyWeakAll { duration } => {
                // Apply Weak to all enemies
                for enemy in &mut self.enemies {
                    enemy.battle_info.apply_weak(*duration);
                }
            },
            BaseEffect::Ethereal { hand_index: _ } => {
                // TODO: Implement ethereal effect
                // Mark card as ethereal (will be exhausted at end of turn)
                // This requires implementing set_ethereal method on deck
            },
            BaseEffect::AddCardToDiscard { card } => {
                // Add a card to the discard pile
                let card_reward_pool = crate::game::card_reward::CardRewardPool::new();
                let created_card = card_reward_pool.create_card_from_enum(*card);
                self.cards.add_card_to_discard(created_card);
            },
            BaseEffect::EnterSelectCardInHand => {
                // Transition to SelectCardInHand state
                self.battle_state = crate::battle::action::BattleState::SelectCardInHand;
            },
            BaseEffect::EnterSelectCardInHandToPutOnDeck => {
                // Transition to SelectCardInHandToPutOnDeck state
                self.battle_state = crate::battle::action::BattleState::SelectCardInHandToPutOnDeck;
            },
            BaseEffect::EnterSelectCardToDuplicate { copies } => {
                // Transition to SelectCardToDuplicate state
                self.battle_state = crate::battle::action::BattleState::SelectCardToDuplicate { copies: *copies };
            },
            BaseEffect::EnterSelectCardInExhaust => {
                // Transition to SelectCardInExhaust state
                self.battle_state = crate::battle::action::BattleState::SelectCardInExhaust;
            },
            BaseEffect::PlayTopCard { source, target } => {
                // Take the top card from draw pile and play it
                if let Some(card) = self.cards.draw_top_card() {
                    // Add the card to hand temporarily to play it
                    let hand_index = self.cards.hand_size();
                    self.cards.add_card_to_hand(card.clone());

                    // Play the card (this will handle cost, effects, etc.)
                    let _ = self.play_card(hand_index, *target);
                }
            },
            BaseEffect::PlayTopCardAndExhaust { source, target } => {
                // Take the top card from draw pile and play it, then exhaust it
                if let Some(card) = self.cards.draw_top_card() {
                    // Add the card to hand temporarily to play it
                    let hand_index = self.cards.hand_size();
                    self.cards.add_card_to_hand(card.clone());

                    // Check if the card has Exhaust effect
                    let has_exhaust = card.get_effects().contains(&crate::game::effect::Effect::Exhaust);

                    // Play the card (this will handle cost, effects, etc.)
                    let _ = self.play_card(hand_index, *target);

                    // If the card doesn't naturally exhaust, exhaust it manually
                    if !has_exhaust {
                        // Find the card in hand and exhaust it
                        if let Some(exhaust_idx) = self.cards.get_hand().iter()
                            .position(|c| c.get_name() == card.get_name() && !c.is_upgraded() == !card.is_upgraded()) {
                            self.cards.exhaust_card_from_hand(exhaust_idx);
                        }
                    }
                }
            },
            BaseEffect::PutCardOnTopOfDrawPile { card } => {
                // Convert the card enum to actual card and put on top of draw pile
                let card_reward_pool = crate::game::card_reward::CardRewardPool::new();
                let created_card = card_reward_pool.create_card_from_enum(*card);
                self.cards.put_card_on_top_of_deck(created_card);
            },
            BaseEffect::PutRandomDiscardCardOnTop => {
                // Take a random card from discard pile and put on top of draw pile
                self.cards.put_random_discard_on_top();
            },
            BaseEffect::EnterSelectCardInDiscard => {
                // Transition to SelectCardInDiscard state
                self.battle_state = crate::battle::action::BattleState::SelectCardInDiscard;
            },
            BaseEffect::ConditionalEffect { condition, effect, source, target } => {
                // Check if the condition is met before applying the effect
                let condition_met = match condition {
                    crate::game::effect::Condition::TargetIsVulnerable => {
                        match target {
                            Entity::Enemy(idx) => {
                                if *idx < self.enemies.len() {
                                    self.enemies[*idx].battle_info.is_vulnerable()
                                } else {
                                    false
                                }
                            },
                            Entity::Player => self.player.battle_info.is_vulnerable(),
                            Entity::None => false,
                        }
                    },
                    crate::game::effect::Condition::HandAllAttacks => {
                        // For conditional effects, this would need to know which player's hand
                        // For now, use current player's hand
                        let hand = self.cards.get_hand();
                        hand.iter().all(|c| c.get_card_type() == &crate::game::card_type::CardType::Attack)
                    },
                    crate::game::effect::Condition::True => true,
                    crate::game::effect::Condition::False => false,
                };

                if condition_met {
                    // Convert the inner effect to BaseEffect and evaluate it
                    let base_effect = crate::game::effect::BaseEffect::from_effect((**effect).clone(), *source, *target);
                    self.eval_base_effect(&base_effect);
                }
            },
            BaseEffect::ExhaustNonAttackCardsFromHand { block_per_card } => {
                // Exhaust all non-Attack cards from hand and gain block per card exhausted
                let hand = self.cards.get_hand().clone();
                let mut cards_to_exhaust = Vec::new();
                let mut num_exhausted = 0u32;

                // Find all non-Attack cards in hand
                for (index, card) in hand.iter().enumerate() {
                    if card.get_card_type() != &crate::game::card_type::CardType::Attack {
                        cards_to_exhaust.push(index);
                        num_exhausted += 1;
                    }
                }

                // Exhaust the non-Attack cards (remove in reverse order to maintain indices)
                for &index in cards_to_exhaust.iter().rev() {
                    self.cards.exhaust_card_from_hand(index);
                }

                // Gain block based on number of cards exhausted
                if num_exhausted > 0 {
                    let total_block = num_exhausted * block_per_card;
                    self.player.battle_info.gain_block(total_block);
                }
            },
            BaseEffect::ActivateRupture => {
                // Activate Rupture listener for gaining Strength when losing HP
                let rupture_listener = crate::cards::ironclad::rupture::RuptureListener::new(crate::battle::target::Entity::Player);
                self.add_listener(Box::new(rupture_listener));
            },
            BaseEffect::ActivateDoubleTap { remaining_attacks } => {
                // Activate DoubleTap listener for playing next Attack(s) twice
                if let Entity::Player = crate::battle::target::Entity::Player {
                    let double_tap_listener = crate::cards::ironclad::double_tap::DoubleTapListener::new(crate::battle::target::Entity::Player, *remaining_attacks);
                    self.add_listener(Box::new(double_tap_listener));
                }
            },
            BaseEffect::HealOnKill { amount } => {
                // Add HealOnKill listener for healing if target dies
                // This will need to track which enemy is being attacked
                // For now, we'll store this in a temporary state
                // TODO: Implement proper HealOnKill listener system
            },
            BaseEffect::AttackAllEnemiesAndHeal { amount, num_attacks } => {
                // Deal damage to all enemies and heal for unblocked damage
                let mut total_unblocked_damage = 0u32;

                for _ in 0..*num_attacks {
                    for enemy_idx in 0..self.enemies.len() {
                        if self.enemies[enemy_idx].battle_info.is_alive() {
                            let target = Entity::Enemy(enemy_idx);
                            let source = Entity::Player; // Assume player is the source
                            let incoming_damage = self.calculate_incoming_damage(source, target, *amount);
                            let actual_damage = self.apply_damage(target, incoming_damage);

                            // Add to total unblocked damage (actual damage is what went through block)
                            total_unblocked_damage += actual_damage;
                        }
                    }
                }

                // Heal the player for total unblocked damage dealt
                if total_unblocked_damage > 0 {
                    let heal_effect = BaseEffect::Heal {
                        target: Entity::Player,
                        amount: total_unblocked_damage,
                    };
                    self.eval_base_effect(&heal_effect);
                }
            },
            BaseEffect::ExhaustHandForDamage { damage_per_card, target } => {
                // Exhaust all cards in hand and deal damage per card exhausted
                let num_cards = self.cards.hand_size() as u32;

                if num_cards > 0 {
                    // Exhaust all cards from hand (in reverse to avoid index shifting)
                    for i in (0..num_cards).rev() {
                        if let Some(_card) = self.cards.exhaust_card_from_hand(i as usize) {
                            // Emit CardExhausted event for each card exhausted
                            let exhaust_event = BattleEvent::CardExhausted {
                                source: *target,
                            };
                            self.emit_event(exhaust_event);
                        }
                    }

                    // Deal damage to first alive enemy (simplified from random selection)
                    for enemy_idx in 0..self.enemies.len() {
                        if self.enemies[enemy_idx].battle_info.is_alive() {
                            let total_damage = num_cards * damage_per_card;
                            let damage_target = Entity::Enemy(enemy_idx);
                            let source = target;

                            let incoming_damage = self.calculate_incoming_damage(*source, damage_target, total_damage);
                            self.apply_damage(damage_target, incoming_damage);
                            break; // Only damage one enemy
                        }
                    }
                }
            },
            // TODO: Implement Juggernaut listener
            // BaseEffect::ActivateJuggernaut { damage_per_block } => {
            //     // Activate Juggernaut listener for dealing damage when gaining block
            //     if let Entity::Player = crate::battle::target::Entity::Player {
            //         let juggernaut_listener = crate::cards::ironclad::juggernaut::JuggernautListener::new(crate::battle::target::Entity::Player, *damage_per_block);
            //         self.add_listener(Box::new(juggernaut_listener));
            //     }
            // },
            BaseEffect::AttackRandomEnemy { amount, num_attacks, strength_multiplier } => {
                // Deal damage to first alive enemy (simplified from random selection)
                for _ in 0..*num_attacks {
                    for enemy_idx in 0..self.enemies.len() {
                        if self.enemies[enemy_idx].battle_info.is_alive() {
                            let target = Entity::Enemy(enemy_idx);
                            let source = Entity::Player;
                            let incoming_damage = self.calculate_incoming_damage(source, target, *amount);

                            // Apply strength multiplier if present
                            let final_amount = if *strength_multiplier > 1 {
                                let strength = self.player.battle_info.get_strength();
                                incoming_damage + ((amount * strength) / 2)
                            } else {
                                incoming_damage
                            };

                            self.apply_damage(target, final_amount);
                            break; // Only damage one enemy
                        }
                    }
                }
            },
            BaseEffect::ActivateFireBreathing { source: _, damage_per_status } => {
                // Activate Fire Breathing listener for dealing damage when Status/Curse cards are drawn
                let fire_breathing_listener = crate::cards::ironclad::fire_breathing::FireBreathingListener::new(crate::battle::target::Entity::Player, *damage_per_status);
                self.add_listener(Box::new(fire_breathing_listener));
            },
            BaseEffect::AddCardToHand { source: _, card } => {
                // Add card to hand
                let card_reward_pool = crate::game::card_reward::CardRewardPool::new();
                let created_card = card_reward_pool.create_card_from_enum(*card);
                self.cards.add_card_to_hand(created_card);
            },
                        BaseEffect::HealAndIncreaseMaxHp { target, amount } => {
                // Heal the target and increase max HP by the same amount
                match *target {
                    Entity::Player => {
                        self.player.battle_info.heal(*amount);
                        self.player.increase_max_hp(*amount);
                    },
                    Entity::Enemy(idx) => {
                        if idx < self.enemies.len() {
                            self.enemies[idx].battle_info.heal(*amount);
                            // Note: Enemies don't have max HP increase in this implementation
                        }
                    }
                    Entity::None => {} // No target, no healing
                };
            },
        }
    }

    /// Apply damage to an entity (player or enemy)
    pub(in crate::battle) fn apply_damage(&mut self, target: Entity, incoming_damage: u32) -> u32 {
        let actual_damage = match target {
            Entity::Player => self.player.battle_info.take_damage(incoming_damage),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.take_damage(incoming_damage)
                } else {
                    0 // Invalid enemy index, no damage dealt
                }
            }
            Entity::None => 0, // No target, no damage dealt
        };
        
        // Emit damage taken event if actual damage was dealt
        if actual_damage > 0 {
            let damage_event = BattleEvent::DamageTaken {
                target,
                amount: actual_damage,
                source: Entity::None, // TODO: Track damage source
            };
            self.emit_event(damage_event);
        }
        
        actual_damage
    }

    /// Apply block to an entity (player or enemy) 
    pub(in crate::battle) fn apply_block(&mut self, target: Entity, amount: u32) {
        match target {
            Entity::Player => self.player.battle_info.gain_block(amount),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.gain_block(amount);
                }
            }
            Entity::None => {} // No target, no block gained
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, card_type::CardType, enemy::EnemyTrait};

    #[test]
    fn test_eval_base_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
            strength_multiplier: 1,
        };
        
        battle.eval_base_effect(&damage_effect);
        
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - 10);
    }

    #[test]
    fn test_attack_all_enemies_effect() {
        use crate::cards::ironclad::cleave::cleave;
        use crate::battle::action::Action;
        
        let mut deck_cards = vec![cleave()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = crate::game::deck::Deck::new(deck_cards);
        
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with multiple enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Draw hand
        battle.cards.draw_n(5);
        
        let initial_enemy1_hp = battle.enemies[0].battle_info.get_hp();
        let initial_enemy2_hp = battle.enemies[1].battle_info.get_hp();
        
        // Find Cleave card in hand
        let hand = battle.cards.get_hand();
        let cleave_idx = hand.iter().position(|card| card.get_name() == "Cleave");
        assert!(cleave_idx.is_some(), "Cleave card should be in hand");
        
        // Play Cleave targeting Entity::None (hits all enemies)
        let action = Action::PlayCard(cleave_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(matches!(result, Ok(_)));
        
        // Both enemies should take 8 damage
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy1_hp - 8);
        assert_eq!(battle.enemies[1].battle_info.get_hp(), initial_enemy2_hp - 8);
    }

    #[test]
    fn test_attack_all_enemies_base_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        // Create battle with multiple enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2))
        ];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_enemy1_hp = battle.enemies[0].battle_info.get_hp();
        let initial_enemy2_hp = battle.enemies[1].battle_info.get_hp();
        
        let attack_all_effect = BaseEffect::AttackAllEnemies {
            source: Entity::Player,
            amount: 8,
            num_attacks: 1,
        };
        
        battle.eval_base_effect(&attack_all_effect);
        
        // Both enemies should take 8 damage
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy1_hp - 8);
        assert_eq!(battle.enemies[1].battle_info.get_hp(), initial_enemy2_hp - 8);
    }

    #[test]
    fn test_vulnerable_effect_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Apply vulnerable to enemy
        let vulnerable_effect = BaseEffect::ApplyVulnerable { target: Entity::Enemy(0), duration: 2 };
        battle.eval_base_effect(&vulnerable_effect);
        
        // Check that enemy is vulnerable
        assert!(battle.enemies[0].battle_info.is_vulnerable());
        assert_eq!(battle.enemies[0].battle_info.get_vulnerable_turns(), 2);
        
        // Apply damage - should be increased by 50%
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
            strength_multiplier: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 10 damage * 1.5 = 15 damage should be dealt (but capped by enemy's HP)
        let expected_damage = 15u32.min(initial_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - expected_damage);
    }

    #[test]
    fn test_character_block_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give enemy some block
        battle.enemies[0].battle_info.gain_block(5);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 5);
        
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 8,
            num_attacks: 1,
            strength_multiplier: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 8 damage - 5 block = 3 actual damage
        // But taking damage triggers Curl Up, giving enemy 3-7 more block (ascension 0)
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 3);
        let curl_up_block = battle.enemies[0].battle_info.get_block();
        assert!(curl_up_block >= 3 && curl_up_block <= 7); // Curl Up activated with random amount
    }

    #[test]
    fn test_damage_to_player() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_hp = battle.player.battle_info.get_hp();
        
        // Create an effect that damages the player
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Enemy(0),
            target: Entity::Player,
            amount: 10,
            num_attacks: 1,
            strength_multiplier: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // Player should take 10 damage
        assert_eq!(battle.player.battle_info.get_hp(), initial_hp - 10);
    }

    #[test]
    fn test_attack_with_strength() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give player some strength
        battle.player.battle_info.gain_strength(3);
        assert_eq!(battle.player.battle_info.get_strength(), 3);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let attack_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 6,
            num_attacks: 1,
            strength_multiplier: 1,
        };
        battle.eval_base_effect(&attack_effect);
        
        // 6 base damage + 3 strength = 9 total damage
        let expected_damage = 9u32.min(initial_enemy_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - expected_damage);
    }

    #[test]
    fn test_add_slimed_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_total_cards = battle.cards.total_cards();
        
        // Apply AddSlimed effect to add 2 Slimed cards
        let add_slimed_effect = BaseEffect::AddSlimed { 
            target: Entity::Player, 
            count: 2 
        };
        battle.eval_base_effect(&add_slimed_effect);
        
        // Should have 2 more cards in discard pile
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + 2);
        assert_eq!(battle.cards.total_cards(), initial_total_cards + 2);
        
        // Check that the added cards are Slimed
        let discard_pile = battle.cards.get_discard_pile();
        let last_two_cards = &discard_pile[discard_pile.len()-2..];
        for card in last_two_cards {
            assert_eq!(card.get_name(), "Slimed");
            assert_eq!(card.get_cost(), 1);
            assert_eq!(card.get_card_type(), &CardType::Status);
        }
    }
}

impl Battle {
    /// Queue an effect to be processed later
    pub(crate) fn queue_effect(&mut self, effect: BaseEffect) {
        self.effect_queue.push(effect);
    }

    /// Process all effects in the effect queue
    pub(crate) fn process_effect_queue(&mut self) {
        while !self.effect_queue.is_empty() {
            // Take the first effect from the queue
            let effect = self.effect_queue.remove(0);

            // Process it (this might add more effects to the queue)
            self.eval_base_effect(&effect);
        }
    }
}