#[derive(Debug, Clone)]
pub struct CharacterBattleInfo {
    pub max_hp: u32,
    pub current_hp: u32,
    pub block: u32,
    pub energy: u32,
    pub vulnerable_turns: u32,
    pub strength: u32,
    pub weak_turns: u32,
    pub ritual: u32,
    // Additional status effects can be added here
}

impl CharacterBattleInfo {
    pub fn new(current_hp: u32, max_hp: u32, energy: u32) -> Self {
        CharacterBattleInfo {
            max_hp,
            current_hp: current_hp.min(max_hp),
            block: 0,
            energy,
            vulnerable_turns: 0,
            strength: 0,
            weak_turns: 0,
            ritual: 0,
        }
    }

    pub fn new_enemy(hp: u32) -> Self {
        CharacterBattleInfo {
            max_hp: hp,
            current_hp: hp,
            block: 0,
            energy: 0, // Enemies don't use energy
            vulnerable_turns: 0,
            strength: 0,
            weak_turns: 0,
            ritual: 0,
        }
    }

    /// Take damage, accounting for block and vulnerable status
    pub fn take_damage(&mut self, base_damage: u32) -> u32 {
        let mut damage = base_damage;
        
        // Apply vulnerable multiplier (50% more damage)
        if self.vulnerable_turns > 0 {
            damage = (damage as f32 * 1.5) as u32;
        }
        
        // Apply block reduction
        let damage_after_block = if damage > self.block {
            let remaining_damage = damage - self.block;
            self.block = 0;
            remaining_damage
        } else {
            self.block -= damage;
            0
        };
        
        // Apply damage to HP
        self.current_hp = self.current_hp.saturating_sub(damage_after_block);
        
        damage_after_block
    }

    /// Gain block (defense)
    pub fn gain_block(&mut self, amount: u32) {
        self.block += amount;
    }

    /// Gain energy
    pub fn gain_energy(&mut self, amount: u32) {
        self.energy += amount;
    }

    /// Spend energy if available
    pub fn spend_energy(&mut self, amount: u32) -> bool {
        if self.energy >= amount {
            self.energy -= amount;
            true
        } else {
            false
        }
    }

    /// Apply vulnerable status (additive)
    pub fn apply_vulnerable(&mut self, turns: u32) {
        self.vulnerable_turns += turns;
    }

    /// Apply weak status (additive)
    pub fn apply_weak(&mut self, turns: u32) {
        self.weak_turns += turns;
    }

    /// Start of turn - reset block and decrement status effects
    pub fn refresh(&mut self) {
        self.block = 0;
        if self.vulnerable_turns > 0 {
            self.vulnerable_turns -= 1;
        }
        if self.weak_turns > 0 {
            self.weak_turns -= 1;
        }
    }

    /// End of turn - apply end-of-turn effects
    pub fn at_end_of_turn(&mut self) {
        // Apply ritual effect (gain strength equal to ritual stacks)
        self.apply_ritual_effect();
    }

    /// Check if character is alive
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    /// Get current HP
    pub fn get_hp(&self) -> u32 {
        self.current_hp
    }

    /// Get max HP
    pub fn get_max_hp(&self) -> u32 {
        self.max_hp
    }

    /// Get block
    pub fn get_block(&self) -> u32 {
        self.block
    }

    /// Get energy
    pub fn get_energy(&self) -> u32 {
        self.energy
    }

    /// Check if vulnerable
    pub fn is_vulnerable(&self) -> bool {
        self.vulnerable_turns > 0
    }

    /// Get vulnerable turns remaining
    pub fn get_vulnerable_turns(&self) -> u32 {
        self.vulnerable_turns
    }

    /// Check if weak
    pub fn is_weak(&self) -> bool {
        self.weak_turns > 0
    }

    /// Get weak turns remaining
    pub fn get_weak_turns(&self) -> u32 {
        self.weak_turns
    }

    /// Gain strength
    pub fn gain_strength(&mut self, amount: u32) {
        self.strength += amount;
    }

    /// Get strength
    pub fn get_strength(&self) -> u32 {
        self.strength
    }

    /// Calculate damage output with strength bonus and weak penalty
    pub fn calculate_damage(&self, base_damage: u32) -> u32 {
        let damage_with_strength = base_damage + self.strength;
        
        // Apply weak penalty (25% less damage)
        if self.weak_turns > 0 {
            (damage_with_strength as f32 * 0.75) as u32
        } else {
            damage_with_strength
        }
    }

    /// Heal HP (up to max)
    pub fn heal(&mut self, amount: u32) {
        self.current_hp = (self.current_hp + amount).min(self.max_hp);
    }

    /// Set max HP
    pub fn set_max_hp(&mut self, hp: u32) {
        self.max_hp = hp;
        // If current HP is now above max, reduce it
        if self.current_hp > self.max_hp {
            self.current_hp = self.max_hp;
        }
    }

    /// Gain ritual stacks
    pub fn gain_ritual(&mut self, amount: u32) {
        self.ritual += amount;
    }

    /// Get ritual stacks
    pub fn get_ritual(&self) -> u32 {
        self.ritual
    }

    /// Apply ritual effect (gain strength equal to ritual stacks)
    pub fn apply_ritual_effect(&mut self) {
        if self.ritual > 0 {
            self.gain_strength(self.ritual);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let character = CharacterBattleInfo::new(50, 50, 3);
        assert_eq!(character.current_hp, 50);
        assert_eq!(character.max_hp, 50);
        assert_eq!(character.energy, 3);
        assert_eq!(character.block, 0);
        assert_eq!(character.vulnerable_turns, 0);
        assert_eq!(character.strength, 0);
        assert_eq!(character.ritual, 0);
    }

    #[test]
    fn test_damage_without_block() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        let damage_dealt = character.take_damage(10);
        assert_eq!(damage_dealt, 10);
        assert_eq!(character.current_hp, 40);
        assert_eq!(character.block, 0);
    }

    #[test]
    fn test_damage_with_block() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.gain_block(5);
        let damage_dealt = character.take_damage(10);
        assert_eq!(damage_dealt, 5);
        assert_eq!(character.current_hp, 45);
        assert_eq!(character.block, 0);
    }

    #[test]
    fn test_damage_blocked_completely() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.gain_block(15);
        let damage_dealt = character.take_damage(10);
        assert_eq!(damage_dealt, 0);
        assert_eq!(character.current_hp, 50);
        assert_eq!(character.block, 5);
    }

    #[test]
    fn test_vulnerable_increases_damage() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.apply_vulnerable(2);
        let damage_dealt = character.take_damage(10);
        assert_eq!(damage_dealt, 15); // 10 * 1.5 = 15
        assert_eq!(character.current_hp, 35);
    }

    #[test]
    fn test_vulnerable_with_block() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.gain_block(5);
        character.apply_vulnerable(1);
        let damage_dealt = character.take_damage(10);
        // 10 * 1.5 = 15 damage, 5 blocked, 10 actual damage
        assert_eq!(damage_dealt, 10);
        assert_eq!(character.current_hp, 40);
        assert_eq!(character.block, 0);
    }

    #[test]
    fn test_start_turn_resets_block_and_decrements_vulnerable() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.gain_block(10);
        character.apply_vulnerable(3);
        
        character.refresh();
        
        assert_eq!(character.block, 0);
        assert_eq!(character.vulnerable_turns, 2);
        
        character.refresh();
        assert_eq!(character.vulnerable_turns, 1);
    }

    #[test]
    fn test_energy_management() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        
        assert!(character.spend_energy(2));
        assert_eq!(character.energy, 1);
        
        assert!(!character.spend_energy(2)); // Not enough energy
        assert_eq!(character.energy, 1);
        
        character.gain_energy(3);
        assert_eq!(character.energy, 4);
    }

    #[test]
    fn test_enemy_character_creation() {
        let character = CharacterBattleInfo::new_enemy(30);
        assert_eq!(character.current_hp, 30);
        assert_eq!(character.max_hp, 30);
        assert_eq!(character.energy, 0); // Enemies have no energy
        assert_eq!(character.block, 0);
        assert_eq!(character.vulnerable_turns, 0);
        assert_eq!(character.strength, 0);
        assert_eq!(character.ritual, 0);
    }

    #[test]
    fn test_strength_mechanics() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        assert_eq!(character.get_strength(), 0);
        
        character.gain_strength(5);
        assert_eq!(character.get_strength(), 5);
        
        // Test damage calculation with strength
        assert_eq!(character.calculate_damage(10), 15); // 10 base + 5 strength
    }

    #[test]
    fn test_strength_accumulation() {
        let mut character = CharacterBattleInfo::new(50, 50, 3);
        character.gain_strength(3);
        character.gain_strength(2);
        assert_eq!(character.get_strength(), 5);
        assert_eq!(character.calculate_damage(8), 13);
    }
}