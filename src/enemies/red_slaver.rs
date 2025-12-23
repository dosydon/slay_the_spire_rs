use crate::game::enemy::EnemyTrait;
use crate::game::effect::BattleEffect;
use crate::game::global_info::GlobalInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedSlaver {
    hp: u32,
    move_history: Vec<RedSlaverMove>,
    has_used_entangle: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedSlaverMove {
    Stab,
    Scrape,
    Entangle,
}

impl RedSlaver {
    /// Calculate Red Slaver's HP based on ascension level
    /// Base: 46-50, A7+: 48-52
    pub fn calculate_hp(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 7 {
            rng.random_range(48..=52)
        } else {
            rng.random_range(46..=50)
        }
    }

    /// Calculate Stab damage based on ascension level
    /// Base: 13, A2+: 14
    pub fn calculate_stab_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            14
        } else {
            13
        }
    }

    /// Calculate Scrape damage based on ascension level
    /// Base: 8, A2+: 9
    pub fn calculate_scrape_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            9
        } else {
            8
        }
    }

    /// Calculate Scrape vulnerable stacks based on ascension level
    /// Base: 1, A17+: 2
    pub fn calculate_scrape_vulnerable(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            2
        } else {
            1
        }
    }

    /// Get the effects for a given move
    pub fn get_move_effects(&self, move_type: RedSlaverMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        match move_type {
            RedSlaverMove::Stab => {
                let damage = Self::calculate_stab_damage(global_info);
                vec![
                    BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                ]
            }
            RedSlaverMove::Scrape => {
                let damage = Self::calculate_scrape_damage(global_info);
                let vulnerable_duration = Self::calculate_scrape_vulnerable(global_info);
                vec![
                    BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                    BattleEffect::ApplyVulnerable { duration: vulnerable_duration },
                ]
            }
            RedSlaverMove::Entangle => {
                vec![
                    BattleEffect::ApplyEntangled { duration: 1 },
                ]
            }
        }
    }

    /// Check if a move can be used based on move history and ascension level
    fn can_use_move(&self, move_type: RedSlaverMove, global_info: &GlobalInfo) -> bool {
        let history_len = self.move_history.len();

        // Pre-Entangle pattern rules (before Entangle is used)
        if !self.has_used_entangle {
            // First move must be Stab
            if history_len == 0 {
                return move_type == RedSlaverMove::Stab;
            }

            // Base/A0-16: Follows "Scrape, Scrape, Stab" pattern until Entangle
            // A17+: Follows "Scrape, Stab" pattern until Entangle
            if global_info.ascention >= 17 {
                // A17+: Cannot use Scrape twice in a row
                if move_type == RedSlaverMove::Scrape
                    && history_len >= 1
                    && self.move_history[history_len - 1] == RedSlaverMove::Scrape {
                    return false;
                }
                // Entangle can be used anytime (25% chance)
                return true;
            } else {
                // Base: "Scrape, Scrape, Stab" pattern (with 25% chance to use Entangle)
                // Entangle can be used anytime (25% chance in choose_move)
                return true;
            }
        }

        // Post-Entangle pattern rules (after Entangle is used)
        // Cannot use Stab 3 times in a row
        if history_len >= 2
            && move_type == RedSlaverMove::Stab
            && self.move_history[history_len - 1] == RedSlaverMove::Stab
            && self.move_history[history_len - 2] == RedSlaverMove::Stab {
            return false;
        }

        // Cannot use Scrape 3 times in a row
        if history_len >= 2
            && move_type == RedSlaverMove::Scrape
            && self.move_history[history_len - 1] == RedSlaverMove::Scrape
            && self.move_history[history_len - 2] == RedSlaverMove::Scrape {
            return false;
        }

        true
    }

    /// Choose the next move according to Red Slaver's pattern
    fn choose_move(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> RedSlaverMove {
        // First move is always Stab
        if self.move_history.is_empty() {
            return RedSlaverMove::Stab;
        }

        // 25% chance to use Entangle (if not used yet)
        if !self.has_used_entangle && rng.random::<f64>() < 0.25 {
            return RedSlaverMove::Entangle;
        }

        // Pre-Entangle pattern
        if !self.has_used_entangle {
            if global_info.ascention >= 17 {
                // A17+: "Scrape, Stab" pattern
                let last_move = self.move_history.last().unwrap();
                match last_move {
                    RedSlaverMove::Stab => RedSlaverMove::Scrape,
                    RedSlaverMove::Scrape => RedSlaverMove::Stab,
                    RedSlaverMove::Entangle => unreachable!(),
                }
            } else {
                // Base: "Scrape, Scrape, Stab" pattern
                let history_len = self.move_history.len();
                if history_len >= 2 {
                    let last_two = &self.move_history[history_len - 2..];
                    if last_two == [RedSlaverMove::Scrape, RedSlaverMove::Scrape] {
                        return RedSlaverMove::Stab;
                    }
                }
                if let Some(last) = self.move_history.last() {
                    if *last == RedSlaverMove::Stab {
                        return RedSlaverMove::Scrape;
                    } else if *last == RedSlaverMove::Scrape {
                        // Check if we've already done one Scrape after Stab
                        if history_len >= 2 && self.move_history[history_len - 2] == RedSlaverMove::Stab {
                            return RedSlaverMove::Scrape; // Second Scrape
                        } else {
                            return RedSlaverMove::Scrape; // Could be first or second
                        }
                    }
                }
                RedSlaverMove::Scrape
            }
        } else {
            // Post-Entangle: 55% Scrape, 45% Stab (with constraints)
            let mut available_moves = Vec::new();
            let mut probabilities = Vec::new();

            if self.can_use_move(RedSlaverMove::Scrape, global_info) {
                available_moves.push(RedSlaverMove::Scrape);
                probabilities.push(0.55);
            }

            if self.can_use_move(RedSlaverMove::Stab, global_info) {
                available_moves.push(RedSlaverMove::Stab);
                probabilities.push(0.45);
            }

            if available_moves.len() == 1 {
                available_moves[0]
            } else {
                let total: f64 = probabilities.iter().sum();
                let normalized: Vec<f64> = probabilities.iter().map(|p| p / total).collect();

                let roll = rng.random::<f64>();
                let mut cumulative = 0.0;
                let mut chosen_move = available_moves[0];

                for (i, prob) in normalized.iter().enumerate() {
                    cumulative += prob;
                    if roll < cumulative {
                        chosen_move = available_moves[i];
                        break;
                    }
                }
                chosen_move
            }
        }
    }
}

impl EnemyTrait for RedSlaver {
    type MoveType = RedSlaverMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let hp = Self::calculate_hp(rng, global_info);
        RedSlaver {
            hp,
            move_history: Vec::new(),
            has_used_entangle: false,
        }
    }

    fn get_name() -> String {
        "Red Slaver".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(
        &mut self,
        global_info: &GlobalInfo,
        rng: &mut impl rand::Rng,
    ) -> (RedSlaverMove, Vec<BattleEffect>) {
        let move_type = self.choose_move(global_info, rng);

        // Mark Entangle as used
        if move_type == RedSlaverMove::Entangle {
            self.has_used_entangle = true;
        }

        // Record the move in history
        self.move_history.push(move_type);

        let effects = self.get_move_effects(move_type, global_info);
        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_slaver_hp_ranges() {
        let mut rng = rand::rng();

        // Base ascension: 46-50 HP
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        for _ in 0..20 {
            let slaver = RedSlaver::instantiate(&mut rng, &global_info_base);
            assert!(slaver.get_hp() >= 46 && slaver.get_hp() <= 50);
        }

        // A7+: 48-52 HP
        let global_info_a7 = GlobalInfo { ascention: 7, current_floor: 1 };
        for _ in 0..20 {
            let slaver = RedSlaver::instantiate(&mut rng, &global_info_a7);
            assert!(slaver.get_hp() >= 48 && slaver.get_hp() <= 52);
        }
    }

    #[test]
    fn test_stab_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_stab_damage(&global_info_base), 13);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_stab_damage(&global_info_a2), 14);
    }

    #[test]
    fn test_scrape_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_scrape_damage(&global_info_base), 8);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_scrape_damage(&global_info_a2), 9);
    }

    #[test]
    fn test_scrape_vulnerable_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_scrape_vulnerable(&global_info_base), 1);

        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        assert_eq!(RedSlaver::calculate_scrape_vulnerable(&global_info_a17), 2);
    }

    #[test]
    fn test_stab_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let slaver = RedSlaver::instantiate(&mut rng, &global_info);

        let effects = slaver.get_move_effects(RedSlaverMove::Stab, &global_info);

        // Should have Attack only
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 13, .. }));
    }

    #[test]
    fn test_scrape_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let slaver = RedSlaver::instantiate(&mut rng, &global_info);

        let effects = slaver.get_move_effects(RedSlaverMove::Scrape, &global_info);

        // Should have Attack and ApplyVulnerable
        assert_eq!(effects.len(), 2);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 8, .. }));
        assert!(matches!(effects[1], BattleEffect::ApplyVulnerable { duration: 1 }));
    }

    #[test]
    fn test_entangle_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let slaver = RedSlaver::instantiate(&mut rng, &global_info);

        let effects = slaver.get_move_effects(RedSlaverMove::Entangle, &global_info);

        // Should have ApplyEntangled only
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::ApplyEntangled { duration: 1 }));
    }

    #[test]
    fn test_first_move_is_stab() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = RedSlaver::instantiate(&mut rng, &global_info);

        // First move must be Stab
        let (move1, _) = slaver.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, RedSlaverMove::Stab);
    }

    #[test]
    fn test_post_entangle_pattern() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = RedSlaver::instantiate(&mut rng, &global_info);

        // Manually set up post-Entangle state
        slaver.has_used_entangle = true;
        slaver.move_history = vec![RedSlaverMove::Stab, RedSlaverMove::Entangle];

        // Count Scrape vs Stab over many moves
        let mut scrape_count = 0;
        let mut stab_count = 0;

        for _ in 0..100 {
            let (move_type, _) = slaver.choose_move_and_effects(&global_info, &mut rng);
            match move_type {
                RedSlaverMove::Scrape => scrape_count += 1,
                RedSlaverMove::Stab => stab_count += 1,
                RedSlaverMove::Entangle => panic!("Should not use Entangle again"),
            }
        }

        // Should be roughly 55% Scrape, 45% Stab (with some tolerance)
        assert!(scrape_count >= 40 && scrape_count <= 70);
        assert!(stab_count >= 30 && stab_count <= 60);
    }

    #[test]
    fn test_cannot_use_stab_three_times_post_entangle() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = RedSlaver::instantiate(&mut rng, &global_info);

        // Set up post-Entangle state with two Stabs in history
        slaver.has_used_entangle = true;
        slaver.move_history = vec![
            RedSlaverMove::Stab,
            RedSlaverMove::Entangle,
            RedSlaverMove::Stab,
            RedSlaverMove::Stab,
        ];

        // Should not be able to use Stab a third time
        assert!(!slaver.can_use_move(RedSlaverMove::Stab, &global_info));
        // But should be able to use Scrape
        assert!(slaver.can_use_move(RedSlaverMove::Scrape, &global_info));
    }

    #[test]
    fn test_cannot_use_scrape_three_times_post_entangle() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = RedSlaver::instantiate(&mut rng, &global_info);

        // Set up post-Entangle state with two Scrapes in history
        slaver.has_used_entangle = true;
        slaver.move_history = vec![
            RedSlaverMove::Stab,
            RedSlaverMove::Entangle,
            RedSlaverMove::Scrape,
            RedSlaverMove::Scrape,
        ];

        // Should not be able to use Scrape a third time
        assert!(!slaver.can_use_move(RedSlaverMove::Scrape, &global_info));
        // But should be able to use Stab
        assert!(slaver.can_use_move(RedSlaverMove::Stab, &global_info));
    }

    #[test]
    fn test_a17_scrape_stab_pattern() {
        let mut rng = rand::rng();
        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        let mut slaver = RedSlaver::instantiate(&mut rng, &global_info_a17);

        // Force no Entangle for pattern testing
        // First move is Stab
        slaver.move_history.push(RedSlaverMove::Stab);

        // At A17+ with pattern "Scrape, Stab", next move should be Scrape
        slaver.move_history.push(RedSlaverMove::Scrape);

        // Cannot use Scrape twice in a row at A17+
        assert!(!slaver.can_use_move(RedSlaverMove::Scrape, &global_info_a17));
    }
}
