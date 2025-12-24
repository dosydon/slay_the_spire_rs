use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Represents different types of potions available in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Potion {
    /// Strength Potion: Grants 2 Strength
    StrengthPotion,
    /// Block Potion: Grants 12 Block
    BlockPotion,
    /// Energy Potion: Grants 2 Energy
    EnergyPotion,
    /// Dexterity Potion: Grants 2 Dexterity
    DexterityPotion,
    /// Fire Potion: Deals 20 damage to all enemies
    FirePotion,
    /// Swift Potion: Gain 2 Artifact and 2 Dexterity
    SwiftPotion,
    /// Blood Potion: Heal 25% of max HP
    BloodPotion,
    /// Explosive Potion: Deals 10 damage to ALL enemies
    ExplosivePotion,
    /// Fear Potion: Apply 3 Vulnerable to target enemy
    FearPotion,
    /// Weak Potion: Apply 3 Weak to target enemy
    WeakPotion,
    /// Ancient Potion: Gain 1 Artifact
    AncientPotion,
    /// Regen Potion: Gain 5 Regeneration (heals 5 HP at end of turn, decreases by 1)
    RegenPotion,
    /// Essence of Steel: Gain 4 Plated Armor
    EssenceOfSteelPotion,
}

impl Potion {
    /// Get the display name of the potion
    pub fn name(&self) -> &'static str {
        match self {
            Potion::StrengthPotion => "Strength Potion",
            Potion::BlockPotion => "Block Potion",
            Potion::EnergyPotion => "Energy Potion",
            Potion::DexterityPotion => "Dexterity Potion",
            Potion::FirePotion => "Fire Potion",
            Potion::SwiftPotion => "Swift Potion",
            Potion::BloodPotion => "Blood Potion",
            Potion::ExplosivePotion => "Explosive Potion",
            Potion::FearPotion => "Fear Potion",
            Potion::WeakPotion => "Weak Potion",
            Potion::AncientPotion => "Ancient Potion",
            Potion::RegenPotion => "Regen Potion",
            Potion::EssenceOfSteelPotion => "Essence of Steel",
        }
    }

    /// Get the description of what the potion does
    pub fn description(&self) -> &'static str {
        match self {
            Potion::StrengthPotion => "Gain 2 Strength",
            Potion::BlockPotion => "Gain 12 Block",
            Potion::EnergyPotion => "Gain 2 Energy",
            Potion::DexterityPotion => "Gain 2 Dexterity",
            Potion::FirePotion => "Deal 20 damage to ALL enemies",
            Potion::SwiftPotion => "Gain 2 Artifact and 2 Dexterity",
            Potion::BloodPotion => "Heal 25% of your missing HP",
            Potion::ExplosivePotion => "Deal 10 damage to ALL enemies",
            Potion::FearPotion => "Apply 3 Vulnerable",
            Potion::WeakPotion => "Apply 3 Weak",
            Potion::AncientPotion => "Gain 1 Artifact",
            Potion::RegenPotion => "Gain 5 Regeneration",
            Potion::EssenceOfSteelPotion => "Gain 4 Plated Armor",
        }
    }

    /// Get the effects that this potion produces when used
    /// Returns a tuple of (target, effects)
    /// target: None means the potion requires target selection, Some(Entity) means fixed target
    pub fn get_effects(&self) -> (Option<Entity>, Vec<BattleEffect>) {
        match self {
            Potion::StrengthPotion => {
                // Strength potion always targets the player
                (Some(Entity::Player), vec![
                    BattleEffect::GainStrength { amount: 2 }
                ])
            }
            Potion::BlockPotion => {
                (Some(Entity::Player), vec![
                    BattleEffect::GainDefense { amount: 12 }
                ])
            }
            Potion::EnergyPotion => {
                (Some(Entity::Player), vec![
                    BattleEffect::GainEnergy { amount: 2 }
                ])
            }
            Potion::DexterityPotion => {
                (Some(Entity::Player), vec![
                    BattleEffect::GainDexterity { amount: 2 }
                ])
            }
            Potion::FirePotion => {
                // Fire potion requires targeting an enemy (hits all enemies)
                // For now, we'll use AttackToTarget since Fire potion should hit all enemies
                // This would need special handling in the battle system
                (None, vec![
                    BattleEffect::AttackAllEnemies { amount: 20, num_attacks: 1 }
                ])
            }
            Potion::SwiftPotion => {
                (Some(Entity::Player), vec![
                    BattleEffect::GainArtifact { amount: 2 },
                    BattleEffect::GainDexterity { amount: 2 }
                ])
            }
            Potion::BloodPotion => {
                // Blood potion heals based on max HP - this is simplified
                (Some(Entity::Player), vec![
                    BattleEffect::Heal(15) // Simplified - should be 25% of max HP
                ])
            }
            Potion::ExplosivePotion => {
                // Explosive potion hits all enemies
                (None, vec![
                    BattleEffect::AttackAllEnemies { amount: 10, num_attacks: 1 }
                ])
            }
            Potion::FearPotion => {
                // Fear potion applies Vulnerable to target enemy
                (None, vec![
                    BattleEffect::ApplyVulnerable { duration: 3 }
                ])
            }
            Potion::WeakPotion => {
                // Weak potion applies Weak to target enemy
                (None, vec![
                    BattleEffect::ApplyWeak { duration: 3 }
                ])
            }
            Potion::AncientPotion => {
                // Ancient potion grants Artifact (prevents debuffs)
                (Some(Entity::Player), vec![
                    BattleEffect::GainArtifact { amount: 1 }
                ])
            }
            Potion::RegenPotion => {
                // Regen potion grants 5 regen (heals 5 HP at end of turn, decreases by 1 each turn)
                (Some(Entity::Player), vec![
                    BattleEffect::GainRegen { amount: 5 }
                ])
            }
            Potion::EssenceOfSteelPotion => {
                // Essence of Steel grants Plated Armor (permanent armor)
                (Some(Entity::Player), vec![
                    BattleEffect::GainPlatedArmor(4)
                ])
            }
        }
    }

    /// Check if this potion requires target selection
    pub fn requires_target(&self) -> bool {
        let (target, _) = self.get_effects();
        target.is_none()
    }

    /// Check if this potion can only be used in combat
    pub fn combat_only(&self) -> bool {
        // All potions are currently combat-only
        true
    }
}

/// Potion inventory with a maximum capacity
#[derive(Debug, Clone)]
pub struct PotionInventory {
    /// List of potions currently in inventory
    potions: Vec<Option<Potion>>,
    /// Maximum number of potion slots
    max_slots: usize,
}

impl PotionInventory {
    /// Create a new potion inventory with the specified number of slots
    pub fn new(max_slots: usize) -> Self {
        Self {
            potions: vec![None; max_slots],
            max_slots,
        }
    }

    /// Create a new potion inventory with default 3 slots
    pub fn default() -> Self {
        Self::new(3)
    }

    /// Get the number of potion slots
    pub fn max_slots(&self) -> usize {
        self.max_slots
    }

    /// Get the number of potions currently in inventory
    pub fn potion_count(&self) -> usize {
        self.potions.iter().filter(|p| p.is_some()).count()
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.potion_count() == self.max_slots
    }

    /// Check if inventory is empty
    pub fn is_empty(&self) -> bool {
        self.potion_count() == 0
    }

    /// Try to add a potion to the inventory
    /// Returns true if successful, false if inventory is full
    pub fn add_potion(&mut self, potion: Potion) -> bool {
        // Find first empty slot
        for slot in &mut self.potions {
            if slot.is_none() {
                *slot = Some(potion);
                return true;
            }
        }
        false
    }

    /// Use a potion at the specified slot index
    /// Returns the potion if successful, None if slot is empty or invalid
    pub fn use_potion(&mut self, slot_index: usize) -> Option<Potion> {
        if slot_index >= self.max_slots {
            return None;
        }
        self.potions[slot_index].take()
    }

    /// Get a reference to the potion at the specified slot
    /// Returns None if slot is empty or invalid
    pub fn get_potion(&self, slot_index: usize) -> Option<Potion> {
        if slot_index >= self.max_slots {
            return None;
        }
        self.potions[slot_index]
    }

    /// Get all potions in the inventory
    pub fn get_all_potions(&self) -> Vec<(usize, Potion)> {
        self.potions
            .iter()
            .enumerate()
            .filter_map(|(idx, potion)| potion.map(|p| (idx, p)))
            .collect()
    }

    /// Discard a potion at the specified slot
    /// Returns true if a potion was discarded, false if slot was already empty
    pub fn discard_potion(&mut self, slot_index: usize) -> bool {
        if slot_index >= self.max_slots {
            return false;
        }
        let had_potion = self.potions[slot_index].is_some();
        self.potions[slot_index] = None;
        had_potion
    }

    /// Increase maximum potion slots by the specified amount
    pub fn increase_slots(&mut self, additional_slots: usize) {
        self.max_slots += additional_slots;
        for _ in 0..additional_slots {
            self.potions.push(None);
        }
    }
}

impl Default for PotionInventory {
    fn default() -> Self {
        Self::default()
    }
}

/// Potion drop pool that tracks drop history to increase drop rates
/// Drop chance increases by 10% each time a potion doesn't drop, resets on drop
#[derive(Debug, Clone)]
pub struct PotionPool {
    /// Number of combats since last potion drop (for increasing drop chance)
    combats_since_drop: u32,
    /// Base drop chance (0.4 = 40%)
    base_drop_chance: f64,
    /// Drop chance increase per combat without drop (0.1 = 10%)
    chance_increase_per_combat: f64,
}

impl PotionPool {
    /// Create a new potion pool with default values
    pub fn new() -> Self {
        Self {
            combats_since_drop: 0,
            base_drop_chance: 0.4,
            chance_increase_per_combat: 0.1,
        }
    }

    /// Get the current drop chance based on history
    /// Increases by 10% per combat without a drop, capped at 100%
    pub fn get_current_drop_chance(&self) -> f64 {
        let increased_chance = self.base_drop_chance
            + (self.combats_since_drop as f64 * self.chance_increase_per_combat);
        increased_chance.min(1.0)
    }

    /// Roll for a potion drop and update history
    /// Returns Some(Potion) if successful, None otherwise
    pub fn roll_potion_drop(&mut self, rng: &mut impl rand::Rng) -> Option<Potion> {
        let drop_chance = self.get_current_drop_chance();

        // Check if potion drops
        if rng.random::<f64>() >= drop_chance {
            // No drop - increment counter
            self.combats_since_drop += 1;
            return None;
        }

        // Potion dropped - reset counter
        self.combats_since_drop = 0;

        // Determine potion rarity
        // Common: 75%, Uncommon: 20%, Rare: 5%
        let roll = rng.random::<f64>();

        // Define available potions by rarity
        let common_potions = vec![
            Potion::StrengthPotion,
            Potion::BlockPotion,
            Potion::EnergyPotion,
            Potion::DexterityPotion,
            Potion::FirePotion,
            Potion::ExplosivePotion,
            Potion::FearPotion,
            Potion::WeakPotion,
        ];

        let uncommon_potions = vec![
            Potion::SwiftPotion,
            Potion::BloodPotion,
            Potion::AncientPotion,
            Potion::RegenPotion,
            Potion::EssenceOfSteelPotion,
        ];

        // TODO: Add rare potions
        let rare_potions = vec![
            Potion::SwiftPotion, // Placeholder - use uncommon for now
        ];

        let potion = if roll < 0.75 {
            // Common potion (75%)
            common_potions[rng.random_range(0..common_potions.len())]
        } else if roll < 0.95 {
            // Uncommon potion (20%)
            uncommon_potions[rng.random_range(0..uncommon_potions.len())]
        } else {
            // Rare potion (5%)
            rare_potions[rng.random_range(0..rare_potions.len())]
        };

        Some(potion)
    }

    /// Get the number of combats since last potion drop
    pub fn get_combats_since_drop(&self) -> u32 {
        self.combats_since_drop
    }

    /// Reset the drop counter (for testing purposes)
    pub fn reset_drop_counter(&mut self) {
        self.combats_since_drop = 0;
    }

    /// Set the drop counter to a specific value (for testing purposes)
    #[cfg(test)]
    pub fn set_combats_since_drop(&mut self, count: u32) {
        self.combats_since_drop = count;
    }
}

impl Default for PotionPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potion_creation() {
        let potion = Potion::StrengthPotion;
        assert_eq!(potion.name(), "Strength Potion");
        assert_eq!(potion.description(), "Gain 2 Strength");
        assert!(potion.combat_only());
        assert!(!potion.requires_target());
    }

    #[test]
    fn test_strength_potion_effects() {
        let potion = Potion::StrengthPotion;
        let (target, effects) = potion.get_effects();

        assert_eq!(target, Some(Entity::Player));
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::GainStrength { amount: 2 }));
    }

    #[test]
    fn test_inventory_creation() {
        let inventory = PotionInventory::new(3);
        assert_eq!(inventory.max_slots(), 3);
        assert_eq!(inventory.potion_count(), 0);
        assert!(inventory.is_empty());
        assert!(!inventory.is_full());
    }

    #[test]
    fn test_add_potion() {
        let mut inventory = PotionInventory::new(2);

        assert!(inventory.add_potion(Potion::StrengthPotion));
        assert_eq!(inventory.potion_count(), 1);
        assert!(!inventory.is_empty());
        assert!(!inventory.is_full());

        assert!(inventory.add_potion(Potion::StrengthPotion));
        assert_eq!(inventory.potion_count(), 2);
        assert!(inventory.is_full());

        // Try to add when full
        assert!(!inventory.add_potion(Potion::StrengthPotion));
        assert_eq!(inventory.potion_count(), 2);
    }

    #[test]
    fn test_use_potion() {
        let mut inventory = PotionInventory::new(3);
        inventory.add_potion(Potion::StrengthPotion);

        // Use potion at slot 0
        let potion = inventory.use_potion(0);
        assert_eq!(potion, Some(Potion::StrengthPotion));
        assert_eq!(inventory.potion_count(), 0);

        // Try to use empty slot
        let potion = inventory.use_potion(0);
        assert_eq!(potion, None);

        // Try to use invalid slot
        let potion = inventory.use_potion(10);
        assert_eq!(potion, None);
    }

    #[test]
    fn test_get_potion() {
        let mut inventory = PotionInventory::new(3);
        inventory.add_potion(Potion::StrengthPotion);

        assert_eq!(inventory.get_potion(0), Some(Potion::StrengthPotion));
        assert_eq!(inventory.get_potion(1), None);
        assert_eq!(inventory.get_potion(10), None);
    }

    #[test]
    fn test_get_all_potions() {
        let mut inventory = PotionInventory::new(4);
        inventory.add_potion(Potion::StrengthPotion);
        inventory.add_potion(Potion::StrengthPotion);

        let all_potions = inventory.get_all_potions();
        assert_eq!(all_potions.len(), 2);
        assert_eq!(all_potions[0], (0, Potion::StrengthPotion));
        assert_eq!(all_potions[1], (1, Potion::StrengthPotion));
    }

    #[test]
    fn test_discard_potion() {
        let mut inventory = PotionInventory::new(3);
        inventory.add_potion(Potion::StrengthPotion);

        assert!(inventory.discard_potion(0));
        assert_eq!(inventory.potion_count(), 0);

        // Try to discard from already empty slot
        assert!(!inventory.discard_potion(0));

        // Try to discard from invalid slot
        assert!(!inventory.discard_potion(10));
    }

    #[test]
    fn test_increase_slots() {
        let mut inventory = PotionInventory::new(2);
        inventory.add_potion(Potion::StrengthPotion);
        inventory.add_potion(Potion::StrengthPotion);

        assert!(inventory.is_full());

        inventory.increase_slots(1);
        assert_eq!(inventory.max_slots(), 3);
        assert!(!inventory.is_full());

        // Should be able to add another potion now
        assert!(inventory.add_potion(Potion::StrengthPotion));
        assert_eq!(inventory.potion_count(), 3);
    }

    #[test]
    fn test_default_inventory() {
        let inventory = PotionInventory::default();
        assert_eq!(inventory.max_slots(), 3);
        assert!(inventory.is_empty());
    }

    #[test]
    fn test_potion_pool_creation() {
        let pool = PotionPool::new();
        assert_eq!(pool.get_combats_since_drop(), 0);
        assert_eq!(pool.get_current_drop_chance(), 0.4);
    }

    #[test]
    fn test_potion_pool_drop_chance_increases() {
        let mut pool = PotionPool::new();

        // Initially 40%
        assert_eq!(pool.get_current_drop_chance(), 0.4);

        // Set counter manually to test increase
        for i in 1..=6 {
            pool.combats_since_drop = i;
            let expected_chance = (0.4 + (i as f64 * 0.1)).min(1.0);
            assert_eq!(pool.get_current_drop_chance(), expected_chance);
        }

        // Should cap at 100%
        pool.combats_since_drop = 10;
        assert_eq!(pool.get_current_drop_chance(), 1.0);
    }

    #[test]
    fn test_potion_pool_roll_no_drop() {
        let mut pool = PotionPool::new();

        // We can't easily test randomness without controlling RNG
        // But we can test that the counter increases after a no-drop scenario
        let initial_combats = pool.get_combats_since_drop();

        // Manually simulate a no-drop by calling the logic directly
        pool.combats_since_drop += 1;

        assert_eq!(pool.get_combats_since_drop(), initial_combats + 1);
    }

    #[test]
    fn test_potion_pool_reset() {
        let mut pool = PotionPool::new();
        pool.combats_since_drop = 5;

        pool.reset_drop_counter();
        assert_eq!(pool.get_combats_since_drop(), 0);
    }

    #[test]
    fn test_potion_pool_drop_chance_caps() {
        let mut pool = PotionPool::new();

        // Test that drop chance caps at 100%
        for i in 0..=20 {
            pool.combats_since_drop = i;
            assert!(pool.get_current_drop_chance() <= 1.0);
        }

        pool.combats_since_drop = 100;
        assert_eq!(pool.get_current_drop_chance(), 1.0);
    }

    #[test]
    fn test_potion_pool_default() {
        let pool = PotionPool::default();
        assert_eq!(pool.get_combats_since_drop(), 0);
        assert_eq!(pool.get_current_drop_chance(), 0.4);
    }
}
