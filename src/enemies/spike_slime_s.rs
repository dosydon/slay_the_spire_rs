use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}, utils::CategoricalDistribution};

#[derive(Clone, Debug)]
pub struct SpikeSlimeS {
    base_damage: u32,
    hp: u32,
}


#[derive(Copy, Debug, Clone, PartialEq)]
pub enum SpikeSlimeSMove {
    Tackle,
}


impl SpikeSlimeS {
    pub fn new(hp: u32, base_damage: u32) -> Self {
        SpikeSlimeS {
            hp,
            base_damage,
        }
    }

    pub fn calculate_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            6
        } else {
            5
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (11, 15)
        } else {
            (10, 14)
        }
    }

    pub fn get_move_effects(&self, move_type: SpikeSlimeSMove) -> Vec<Effect> {
        match move_type {
            SpikeSlimeSMove::Tackle => {
                vec![Effect::AttackToTarget { amount: self.base_damage, num_attacks: 1 }]
            }
        }
    }


    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<SpikeSlimeSMove> {
        // Spike Slime only has one move - Tackle
        let outcomes_and_weights = vec![(SpikeSlimeSMove::Tackle, 1.0)];
        CategoricalDistribution::new(outcomes_and_weights)
    }

}

impl EnemyTrait for SpikeSlimeS {
    type MoveType = SpikeSlimeSMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        let base_damage = Self::calculate_damage(global_info);
        
        SpikeSlimeS::new(hp, base_damage)
    }

    fn get_name() -> String {
        "Spike Slime (S)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (SpikeSlimeSMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);
        
        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move);
        
        (selected_move, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_spike_slime_creation() {
        let spike_slime = SpikeSlimeS::new(12, 5);
        assert_eq!(spike_slime.hp, 12);
        assert_eq!(spike_slime.base_damage, 5);
    }

    #[test]
    fn test_spike_slime_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(SpikeSlimeS::calculate_damage(&global_info_asc0), 5);
        assert_eq!(SpikeSlimeS::calculate_damage(&global_info_asc2), 6);

        // Test HP scaling
        assert_eq!(SpikeSlimeS::calculate_hp_range(&global_info_asc0), (10, 14));
        assert_eq!(SpikeSlimeS::calculate_hp_range(&global_info_asc7), (11, 15));
    }

    #[test]
    fn test_spike_slime_move_pattern() {
        let mut spike_slime = SpikeSlimeS::new(12, 5);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Should always choose Tackle (only move available)
        for _ in 0..10 {
            let (move_type, effects) = spike_slime.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(move_type, SpikeSlimeSMove::Tackle);
            assert_eq!(effects, vec![Effect::AttackToTarget { amount: 5, num_attacks: 1 }]);
        }
    }

    #[test]
    fn test_spike_slime_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let spike_slime_asc0 = SpikeSlimeS::instantiate(&mut rng, &global_info_asc0);
        assert!(spike_slime_asc0.hp >= 10 && spike_slime_asc0.hp <= 14);
        assert_eq!(spike_slime_asc0.base_damage, 5);

        // Test high ascension instantiation
        let spike_slime_asc7 = SpikeSlimeS::instantiate(&mut rng, &global_info_asc7);
        assert!(spike_slime_asc7.hp >= 11 && spike_slime_asc7.hp <= 15);
        assert_eq!(spike_slime_asc7.base_damage, 6); // A7 is >= A2, so damage is 6
    }

    #[test]
    fn test_spike_slime_effects() {
        let spike_slime = SpikeSlimeS::new(12, 5);

        // Test Tackle effects
        let tackle_effects = spike_slime.get_move_effects(SpikeSlimeSMove::Tackle);
        assert_eq!(tackle_effects, vec![Effect::AttackToTarget { amount: 5, num_attacks: 1 }]);
    }
}