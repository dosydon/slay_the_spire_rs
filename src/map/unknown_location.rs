use crate::map::node::NodeType;
use rand::Rng;

/// Tracks the sampling state for Unknown Locations (? rooms)
///
/// Each time a ? room is visited and doesn't contain a particular type of encounter,
/// the probability of that encounter type increases for the next ? room.
/// When an encounter type is found, its probability resets to the base value.
///
/// Note: ? rooms are stored as NodeType::Event on the map and are sampled when visited.
#[derive(Debug, Clone)]
pub struct UnknownLocationSampler {
    /// Number of ? rooms since last Monster encounter
    pub monster_count: u32,
    /// Number of ? rooms since last Treasure encounter
    pub treasure_count: u32,
    /// Number of ? rooms since last Shop encounter
    pub shop_count: u32,
    /// Set of Event node IDs that are actually ? rooms (to be sampled when visited)
    unknown_room_ids: Vec<(u32, u32)>,
}

impl UnknownLocationSampler {
    /// Create a new sampler with default values
    pub fn new() -> Self {
        Self {
            monster_count: 0,
            treasure_count: 0,
            shop_count: 0,
            unknown_room_ids: Vec::new(),
        }
    }

    /// Register an Event node as a ? room that should be sampled when visited
    pub fn register_unknown_room(&mut self, node_id: (u32, u32)) {
        self.unknown_room_ids.push(node_id);
    }

    /// Check if a node ID is a ? room that needs sampling
    pub fn is_unknown_room(&self, node_id: (u32, u32)) -> bool {
        self.unknown_room_ids.contains(&node_id)
    }

    /// Get the current probability for Monster encounters
    /// Base 10%, increases by 10% for each ? room without a Monster
    fn get_monster_probability(&self) -> f64 {
        let base_prob = 0.10; // 10%
        let increment = 0.10 * self.monster_count as f64; // +10% per ? room
        (base_prob + increment).min(1.0)
    }

    /// Get the current probability for Treasure encounters
    /// Base 2%, increases by 2% for each ? room without Treasure
    fn get_treasure_probability(&self) -> f64 {
        let base_prob = 0.02; // 2%
        let increment = 0.02 * self.treasure_count as f64; // +2% per ? room
        (base_prob + increment).min(1.0)
    }

    /// Get the current probability for Shop encounters
    /// Base 3%, increases by 3% for each ? room without a Shop
    fn get_shop_probability(&self) -> f64 {
        let base_prob = 0.03; // 3%
        let increment = 0.03 * self.shop_count as f64; // +3% per ? room
        (base_prob + increment).min(1.0)
    }

    /// Sample the encounter type for an Unknown Location (? room)
    ///
    /// The sampling works as follows:
    /// 1. Roll for Monster (if successful, return Combat and reset monster counter)
    /// 2. Roll for Treasure (if successful, return Treasure and reset treasure counter)
    /// 3. Roll for Shop (if successful, return Shop and reset shop counter)
    /// 4. If nothing else was selected, return Event (default)
    ///
    /// Returns the NodeType that should replace the Unknown node
    pub fn sample_unknown_location(&mut self, rng: &mut impl Rng) -> NodeType {
        // Record that we're visiting a ? room
        self.record_unknown_room_visited();

        // Roll for Monster
        if rng.random::<f64>() < self.get_monster_probability() {
            self.monster_count = 0; // Reset monster counter
            // Note: we don't reset other counters
            return NodeType::Combat;
        }

        // Roll for Treasure
        if rng.random::<f64>() < self.get_treasure_probability() {
            self.treasure_count = 0; // Reset treasure counter
            // Note: we don't reset other counters
            return NodeType::Treasure;
        }

        // Roll for Shop
        if rng.random::<f64>() < self.get_shop_probability() {
            self.shop_count = 0; // Reset shop counter
            // Note: we don't reset other counters
            return NodeType::Shop;
        }

        // If nothing was selected, default to Event
        // Event doesn't have a probability - it's the fallback
        NodeType::Event
    }

    /// Record that a ? room was visited
    /// This increments all counters (they reset when their type is actually found)
    fn record_unknown_room_visited(&mut self) {
        self.monster_count += 1;
        self.treasure_count += 1;
        self.shop_count += 1;
    }

    /// Reset all counters (used when starting a new act)
    pub fn reset(&mut self) {
        self.monster_count = 0;
        self.treasure_count = 0;
        self.shop_count = 0;
        self.unknown_room_ids.clear();
    }
}

impl Default for UnknownLocationSampler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_creation() {
        let sampler = UnknownLocationSampler::new();
        assert_eq!(sampler.monster_count, 0);
        assert_eq!(sampler.treasure_count, 0);
        assert_eq!(sampler.shop_count, 0);
    }

    #[test]
    fn test_monster_probability_increases() {
        let mut sampler = UnknownLocationSampler::new();

        // Initial probability should be 10%
        assert!((sampler.get_monster_probability() - 0.10).abs() < 0.001);

        // After 1 ? room without monster: 20%
        sampler.monster_count = 1;
        assert!((sampler.get_monster_probability() - 0.20).abs() < 0.001);

        // After 2 ? rooms without monster: 30%
        sampler.monster_count = 2;
        assert!((sampler.get_monster_probability() - 0.30).abs() < 0.001);

        // After 9 ? rooms: should cap at 100%
        sampler.monster_count = 9;
        assert_eq!(sampler.get_monster_probability(), 1.0);

        // After 10 ? rooms: still capped at 100%
        sampler.monster_count = 10;
        assert_eq!(sampler.get_monster_probability(), 1.0);
    }

    #[test]
    fn test_treasure_probability_increases() {
        let mut sampler = UnknownLocationSampler::new();

        // Initial probability should be 2%
        assert_eq!(sampler.get_treasure_probability(), 0.02);

        // After 1 ? room without treasure: 4%
        sampler.treasure_count = 1;
        assert_eq!(sampler.get_treasure_probability(), 0.04);

        // After 2 ? rooms without treasure: 6%
        sampler.treasure_count = 2;
        assert_eq!(sampler.get_treasure_probability(), 0.06);
    }

    #[test]
    fn test_shop_probability_increases() {
        let mut sampler = UnknownLocationSampler::new();

        // Initial probability should be 3%
        assert_eq!(sampler.get_shop_probability(), 0.03);

        // After 1 ? room without shop: 6%
        sampler.shop_count = 1;
        assert_eq!(sampler.get_shop_probability(), 0.06);

        // After 2 ? rooms without shop: 9%
        sampler.shop_count = 2;
        assert_eq!(sampler.get_shop_probability(), 0.09);
    }

    #[test]
    fn test_record_unknown_room_visited() {
        let mut sampler = UnknownLocationSampler::new();

        assert_eq!(sampler.monster_count, 0);
        assert_eq!(sampler.treasure_count, 0);
        assert_eq!(sampler.shop_count, 0);

        sampler.record_unknown_room_visited();

        assert_eq!(sampler.monster_count, 1);
        assert_eq!(sampler.treasure_count, 1);
        assert_eq!(sampler.shop_count, 1);
    }

    #[test]
    fn test_reset() {
        let mut sampler = UnknownLocationSampler::new();

        sampler.monster_count = 5;
        sampler.treasure_count = 3;
        sampler.shop_count = 7;

        sampler.reset();

        assert_eq!(sampler.monster_count, 0);
        assert_eq!(sampler.treasure_count, 0);
        assert_eq!(sampler.shop_count, 0);
    }
}
