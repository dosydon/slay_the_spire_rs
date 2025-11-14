use crate::{battle::{events::{BattleEvent, EventListener}, target::Entity}, game::effect::Effect};

#[derive(Debug)]
pub struct CurlUpListener {
    used: bool,
    block_amount: u32,
    owner: Entity,
}

impl CurlUpListener {
    pub(in crate::battle) fn new(owner: Entity, ascension_level: u32, rng: &mut impl rand::Rng) -> Self {
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

#[derive(Debug)]
pub struct EnrageListener {
    enrage_amount: u32,
    owner: Entity,
}

impl EnrageListener {
    pub(in crate::battle) fn new(owner: Entity, enrage_amount: u32) -> Self {
        EnrageListener {
            enrage_amount,
            owner,
        }
    }
}

impl EventListener for EnrageListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::SkillCardPlayed { source } if *source == Entity::Player => {
                // When the player plays a Skill card, the Gremlin Nob (this listener's owner) gains Strength
                vec![Effect::GainStrength(self.enrage_amount)]
            }
            _ => vec![]
        }
    }
    
    fn is_active(&self) -> bool {
        true // Enrage is always active
    }
    
    fn get_owner(&self) -> Entity {
        self.owner
    }
}

#[cfg(test)]
mod enrage_tests {
    use super::*;

    #[test]
    fn test_enrage_listener_creation() {
        let listener = EnrageListener::new(Entity::Enemy(0), 2);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
        assert_eq!(listener.enrage_amount, 2);
    }

    #[test]
    fn test_enrage_triggers_on_skill_card() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 2);
        
        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        
        let effects = listener.on_event(&skill_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainStrength(2));
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_enrage_does_not_trigger_on_damage() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 2);
        
        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 6,
            source: Entity::Player,
        };
        
        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_enrage_triggers_multiple_times() {
        let mut listener = EnrageListener::new(Entity::Enemy(0), 3);
        
        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        
        // First skill card
        let effects1 = listener.on_event(&skill_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], Effect::GainStrength(3));
        
        // Second skill card should also trigger
        let effects2 = listener.on_event(&skill_event);
        assert_eq!(effects2.len(), 1);
        assert_eq!(effects2[0], Effect::GainStrength(3));
        
        assert!(listener.is_active()); // Always active
    }

    #[test]
    fn test_enrage_different_amounts() {
        let mut listener_2 = EnrageListener::new(Entity::Enemy(0), 2);
        let mut listener_3 = EnrageListener::new(Entity::Enemy(1), 3);
        
        let skill_event = BattleEvent::SkillCardPlayed {
            source: Entity::Player,
        };
        
        let effects_2 = listener_2.on_event(&skill_event);
        assert_eq!(effects_2, vec![Effect::GainStrength(2)]);
        
        let effects_3 = listener_3.on_event(&skill_event);
        assert_eq!(effects_3, vec![Effect::GainStrength(3)]);
    }
}

#[derive(Debug)]
pub struct LoseStrengthListener {
    amount_to_lose: u32,
    owner: Entity,
    is_active: bool,
}

impl LoseStrengthListener {
    pub(in crate::battle) fn new(owner: Entity, amount_to_lose: u32) -> Self {
        LoseStrengthListener {
            amount_to_lose,
            owner,
            is_active: true,
        }
    }
}

impl EventListener for LoseStrengthListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                self.is_active = false; // Only trigger once
                vec![Effect::LoseStrength(self.amount_to_lose)]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}
#[cfg(test)]
mod lose_strength_tests {
    use super::*;

    #[test]
    fn test_lose_strength_listener_creation() {
        let listener = LoseStrengthListener::new(Entity::Player, 3);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_lose_strength_triggers_on_end_of_turn() {
        let mut listener = LoseStrengthListener::new(Entity::Player, 2);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::LoseStrength(2));
        assert!(!listener.is_active()); // Used up
    }

    #[test]
    fn test_lose_strength_only_triggers_once() {
        let mut listener = LoseStrengthListener::new(Entity::Enemy(0), 1);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(0),
        };

        // First end turn triggers strength loss
        let effects1 = listener.on_event(&end_turn_event);
        assert_eq!(effects1.len(), 1);
        assert_eq!(effects1[0], Effect::LoseStrength(1));

        // Second end turn should not trigger
        let effects2 = listener.on_event(&end_turn_event);
        assert_eq!(effects2.len(), 0);
    }

    #[test]
    fn test_lose_strength_wrong_entity() {
        let mut listener = LoseStrengthListener::new(Entity::Enemy(0), 2);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(1), // Different enemy
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active()); // Still active
    }

    #[test]
    fn test_lose_strength_different_amounts() {
        let mut listener_2 = LoseStrengthListener::new(Entity::Player, 2);
        let mut listener_5 = LoseStrengthListener::new(Entity::Player, 5);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects_2 = listener_2.on_event(&end_turn_event);
        assert_eq!(effects_2, vec![Effect::LoseStrength(2)]);

        let effects_5 = listener_5.on_event(&end_turn_event);
        assert_eq!(effects_5, vec![Effect::LoseStrength(5)]);
    }
}
