use crate::game::enemy::EnemyTrait;
use crate::game::effect::Effect;
use crate::game::global_info::GlobalInfo;
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FungiBeast {
    pub hp: u32,
    strength: u32,
    last_move: Option<FungiBeastMove>,
    consecutive_bites: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FungiBeastMove {
    Bite,
    Grow,
}

impl FungiBeast {
    /// Calculate Fungi Beast's HP based on ascension level
    /// Base: 22-28, A7+: 24-28
    pub fn calculate_hp(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 7 {
            rng.random_range(24..=28)
        } else {
            rng.random_range(22..=28)
        }
    }

    /// Calculate Bite damage
    /// Always 6 damage (no ascension scaling)
    pub fn calculate_bite_damage() -> u32 {
        6
    }

    /// Calculate Grow strength gain based on ascension level
    /// Base: 3, A2+: 4, A17+: 5
    pub fn calculate_grow_strength(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            5
        } else if global_info.ascention >= 2 {
            4
        } else {
            3
        }
    }

    /// Calculate Spore Cloud vulnerable stacks
    /// Always 2 stacks (no ascension scaling based on wiki)
    pub fn calculate_spore_cloud_vulnerable() -> u32 {
        2
    }

    /// Get the effects for a given move
    pub fn get_move_effects(&self, move_type: FungiBeastMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            FungiBeastMove::Bite => {
                let damage = Self::calculate_bite_damage();
                vec![Effect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 }]
            }
            FungiBeastMove::Grow => {
                let strength = Self::calculate_grow_strength(global_info);
                vec![Effect::GainStrength { amount: strength }]
            }
        }
    }

    /// Get the on-death effects (Spore Cloud)
    pub fn get_on_death_effects() -> Vec<Effect> {
        let vulnerable = Self::calculate_spore_cloud_vulnerable();
        vec![Effect::ApplyVulnerable { duration: vulnerable }]
    }
}

/// Spore Cloud listener - applies Vulnerable when Fungi Beast dies
pub struct SporeCloudListener {
    owner: Entity,
    active: bool,
}

impl SporeCloudListener {
    pub fn new(enemy_index: usize) -> Self {
        SporeCloudListener {
            owner: Entity::Enemy(enemy_index),
            active: true,
        }
    }
}

impl EventListener for SporeCloudListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        if !self.active {
            return vec![];
        }

        match event {
            BattleEvent::EnemyDeath { enemy } => {
                if enemy == &self.owner {
                    // This Fungi Beast has died - apply Spore Cloud
                    self.active = false; // One-time effect
                    FungiBeast::get_on_death_effects()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl EnemyTrait for FungiBeast {
    type MoveType = FungiBeastMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let hp = Self::calculate_hp(rng, global_info);
        FungiBeast {
            hp,
            strength: 0,
            last_move: None,
            consecutive_bites: 0,
        }
    }

    fn get_name() -> String {
        "Fungi Beast".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(
        &mut self,
        global_info: &GlobalInfo,
        rng: &mut impl rand::Rng,
    ) -> (FungiBeastMove, Vec<Effect>) {
        let move_type = match self.last_move {
            None => {
                // First turn: use weighted random (60% Bite, 40% Grow)
                if rng.random::<f64>() < 0.6 {
                    self.consecutive_bites = 1;
                    FungiBeastMove::Bite
                } else {
                    self.consecutive_bites = 0;
                    FungiBeastMove::Grow
                }
            }
            Some(FungiBeastMove::Bite) => {
                if self.consecutive_bites >= 2 {
                    // Must use Grow (cannot bite 3 times in a row)
                    self.consecutive_bites = 0;
                    FungiBeastMove::Grow
                } else {
                    // Can choose: 60% Bite, 40% Grow
                    if rng.random::<f64>() < 0.6 {
                        self.consecutive_bites += 1;
                        FungiBeastMove::Bite
                    } else {
                        self.consecutive_bites = 0;
                        FungiBeastMove::Grow
                    }
                }
            }
            Some(FungiBeastMove::Grow) => {
                // Cannot grow twice in a row, must Bite
                self.consecutive_bites = 1;
                FungiBeastMove::Bite
            }
        };

        self.last_move = Some(move_type);
        let effects = self.get_move_effects(move_type, global_info);
        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fungi_beast_hp_ranges() {
        let mut rng = rand::rng();

        // Base ascension: 22-28 HP
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        for _ in 0..20 {
            let fungi = FungiBeast::instantiate(&mut rng, &global_info_base);
            assert!(fungi.get_hp() >= 22 && fungi.get_hp() <= 28);
        }

        // A7+: 24-28 HP
        let global_info_a7 = GlobalInfo { ascention: 7, current_floor: 1 };
        for _ in 0..20 {
            let fungi = FungiBeast::instantiate(&mut rng, &global_info_a7);
            assert!(fungi.get_hp() >= 24 && fungi.get_hp() <= 28);
        }
    }

    #[test]
    fn test_bite_damage() {
        assert_eq!(FungiBeast::calculate_bite_damage(), 6);
    }

    #[test]
    fn test_grow_strength_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(FungiBeast::calculate_grow_strength(&global_info_base), 3);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(FungiBeast::calculate_grow_strength(&global_info_a2), 4);

        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        assert_eq!(FungiBeast::calculate_grow_strength(&global_info_a17), 5);
    }

    #[test]
    fn test_spore_cloud_vulnerable() {
        assert_eq!(FungiBeast::calculate_spore_cloud_vulnerable(), 2);
    }

    #[test]
    fn test_cannot_bite_three_times() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut fungi = FungiBeast::instantiate(&mut rng, &global_info);

        // Force consecutive bites by setting state
        fungi.last_move = Some(FungiBeastMove::Bite);
        fungi.consecutive_bites = 2;

        // Next move MUST be Grow
        let (move_chosen, _) = fungi.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move_chosen, FungiBeastMove::Grow);
    }

    #[test]
    fn test_cannot_grow_twice() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut fungi = FungiBeast::instantiate(&mut rng, &global_info);

        // Force last move to be Grow
        fungi.last_move = Some(FungiBeastMove::Grow);

        // Next move MUST be Bite
        let (move_chosen, _) = fungi.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move_chosen, FungiBeastMove::Bite);
    }

    #[test]
    fn test_bite_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let fungi = FungiBeast::instantiate(&mut rng, &global_info);

        let effects = fungi.get_move_effects(FungiBeastMove::Bite, &global_info);

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::AttackToTarget { amount: 6, .. }));
    }

    #[test]
    fn test_grow_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let fungi = FungiBeast::instantiate(&mut rng, &global_info);

        let effects = fungi.get_move_effects(FungiBeastMove::Grow, &global_info);

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::GainStrength { amount: 3 }));
    }

    #[test]
    fn test_on_death_effects() {
        let effects = FungiBeast::get_on_death_effects();

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::ApplyVulnerable { duration: 2 }));
    }

    #[test]
    fn test_move_pattern_probabilities() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let mut bite_count = 0;
        let mut grow_count = 0;
        let iterations = 1000;

        for _ in 0..iterations {
            let mut fungi = FungiBeast::instantiate(&mut rng, &global_info);
            let (first_move, _) = fungi.choose_move_and_effects(&global_info, &mut rng);

            match first_move {
                FungiBeastMove::Bite => bite_count += 1,
                FungiBeastMove::Grow => grow_count += 1,
            }
        }

        // First move should be roughly 60% Bite, 40% Grow
        let bite_ratio = bite_count as f64 / iterations as f64;
        assert!(bite_ratio > 0.55 && bite_ratio < 0.65, "Bite ratio {} should be around 0.60", bite_ratio);
    }
}
