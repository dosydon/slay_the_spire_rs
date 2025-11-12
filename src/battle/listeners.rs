use crate::{battle::{events::{BattleEvent, EventListener}, target::Entity}, game::effect::Effect};

#[derive(Debug)]
pub struct CurlUpListener {
    used: bool,
    block_amount: u32,
    owner: Entity,
}

impl CurlUpListener {
    pub fn new(owner: Entity, ascension_level: u32, rng: &mut impl rand::Rng) -> Self {
        let block_amount = match ascension_level {
            a if a >= 17 => rng.random_range(9..=12), // A17+ gives 9-12 block
            a if a >= 7 => rng.random_range(4..=8),   // A7-16 gives 4-8 block  
            _ => rng.random_range(3..=7),             // A0-6 gives 3-7 block
        };
        
        CurlUpListener {
            used: false,
            block_amount,
            owner,
        }
    }
}

impl EventListener for CurlUpListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::DamageTaken { target, amount, .. } 
                if *target == self.owner && *amount > 0 && !self.used => {
                self.used = true;
                vec![Effect::GainDefense(self.block_amount)]
            }
            _ => vec![]
        }
    }
    
    fn is_active(&self) -> bool {
        !self.used
    }
    
    fn get_owner(&self) -> Entity {
        self.owner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curl_up_listener_creation() {
        let mut rng = rand::rng();
        let listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
        // Block amount should be in range 3-7 for ascension 0
        assert!(listener.block_amount >= 3 && listener.block_amount <= 7);
    }

    #[test]
    fn test_curl_up_listener_ascension() {
        let mut rng = rand::rng();
        let normal_listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        let mid_ascension_listener = CurlUpListener::new(Entity::Enemy(0), 10, &mut rng);
        let high_ascension_listener = CurlUpListener::new(Entity::Enemy(0), 17, &mut rng);
        
        // Test ascension scaling ranges
        assert!(normal_listener.block_amount >= 3 && normal_listener.block_amount <= 7);
        assert!(mid_ascension_listener.block_amount >= 4 && mid_ascension_listener.block_amount <= 8);
        assert!(high_ascension_listener.block_amount >= 9 && high_ascension_listener.block_amount <= 12);
    }

    #[test]
    fn test_curl_up_triggers_on_damage() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        let expected_block = listener.block_amount; // Store the randomly generated amount
        
        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };
        
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainDefense(expected_block));
        assert!(!listener.is_active()); // Used up
    }

    #[test]
    fn test_curl_up_only_triggers_once() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        
        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };
        
        // First damage triggers curl up
        listener.on_event(&damage_event);
        
        // Second damage should not trigger
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_curl_up_wrong_target() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        
        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(1), // Different target
            amount: 6,
            source: Entity::Player,
        };
        
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }

    #[test]
    fn test_curl_up_zero_damage() {
        let mut rng = rand::rng();
        let mut listener = CurlUpListener::new(Entity::Enemy(0), 0, &mut rng);
        
        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 0, // No damage
            source: Entity::Player,
        };
        
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }
}