/// Player state that persists across battles during a run
/// This includes HP, gold, relics, and potions
#[derive(Debug, Clone)]
pub struct PlayerRunState {
    /// Current HP
    pub current_hp: u32,
    /// Maximum HP
    pub max_hp: u32,
    /// Current gold
    pub gold: u32,
    /// Relics collected during the run
    pub relics: Vec<crate::relics::Relic>,
    /// Potion inventory
    pub potions: crate::potion::PotionInventory,
}

impl PlayerRunState {
    /// Create a new PlayerRunState with starting values
    pub fn new(current_hp: u32, max_hp: u32, gold: u32) -> Self {
        Self {
            current_hp,
            max_hp,
            gold,
            relics: Vec::new(),
            potions: crate::potion::PotionInventory::default(),
        }
    }

    /// Create a new PlayerRunState with relics
    pub fn new_with_relics(current_hp: u32, max_hp: u32, gold: u32, relics: Vec<crate::relics::Relic>) -> Self {
        Self {
            current_hp,
            max_hp,
            gold,
            relics,
            potions: crate::potion::PotionInventory::default(),
        }
    }

    /// Create a new PlayerRunState with relics and potions
    pub fn new_with_relics_and_potions(
        current_hp: u32,
        max_hp: u32,
        gold: u32,
        relics: Vec<crate::relics::Relic>,
        potions: crate::potion::PotionInventory,
    ) -> Self {
        Self {
            current_hp,
            max_hp,
            gold,
            relics,
            potions,
        }
    }
}
