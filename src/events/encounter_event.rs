use crate::game::enemy::EnemyTrait;
use crate::enemies::EnemyEnum;

pub enum EncounterEvent {
    TwoLouses,
    JawWorm
}

impl EncounterEvent {
    pub fn instantiate(&self, rng: &mut impl rand::Rng, global_info: &crate::game::global_info::GlobalInfo) -> Vec<EnemyEnum> {
        match self {
            EncounterEvent::TwoLouses => {
                use rand::Rng;
                
                // First louse: 50% chance of red or green
                let louse1 = if rng.random::<f64>() < 0.5 {
                    let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                    EnemyEnum::RedLouse(red_louse)
                } else {
                    let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                    EnemyEnum::GreenLouse(green_louse)
                };
                
                // Second louse: 50% chance of red or green
                let louse2 = if rng.random::<f64>() < 0.5 {
                    let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                    EnemyEnum::RedLouse(red_louse)
                } else {
                    let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                    EnemyEnum::GreenLouse(green_louse)
                };
                
                vec![louse1, louse2]
            }
            EncounterEvent::JawWorm => {
                let jaw_worm = crate::enemies::jaw_worm::JawWorm::instantiate(rng, global_info);
                vec![EnemyEnum::JawWorm(jaw_worm)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_jaw_worm_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        let encounter = EncounterEvent::JawWorm;
        let enemies = encounter.instantiate(&mut rng, &global_info);
        
        assert_eq!(enemies.len(), 1);
        
        match &enemies[0] {
            EnemyEnum::JawWorm(_) => {
                // Success - we got a JawWorm
            }
            _ => panic!("Expected JawWorm enemy"),
        }
    }

    #[test]
    fn test_two_louses_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        let encounter = EncounterEvent::TwoLouses;
        let enemies = encounter.instantiate(&mut rng, &global_info);
        
        assert_eq!(enemies.len(), 2);
        
        // Both should be louses (either red or green)
        for enemy in &enemies {
            match enemy {
                EnemyEnum::RedLouse(_) | EnemyEnum::GreenLouse(_) => {
                    // Success - we got a louse
                }
                _ => panic!("Expected RedLouse or GreenLouse enemy"),
            }
        }
    }
}

