/// Types of encounters available on the map
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    /// Combat encounter with enemies
    Combat,
    /// Elite combat encounter with stronger enemies and better rewards
    Elite,
    /// Rest site where player can heal or upgrade cards
    RestSite,
    /// Random event with choices and consequences
    /// (Also used for Unknown Location ? rooms before sampling)
    Event,
    /// Shop where player can buy cards, relics, and potions
    Shop,
    /// Treasure room with guaranteed rewards
    Treasure,
    /// Boss encounter (usually at the end of acts)
    Boss,
    /// Starting node (beginning of the run)
    Start,
}

/// A node in the map graph representing an encounter
#[derive(Debug, Clone, PartialEq)]
pub struct MapNode {
    /// Floor level (0-based)
    pub floor: u32,
    /// Position on the floor (0-based, left to right)
    pub position: u32,
    /// Type of encounter at this node
    pub node_type: NodeType,
}

impl MapNode {
    pub fn new(floor: u32, position: u32, node_type: NodeType) -> Self {
        MapNode {
            floor,
            position,
            node_type,
        }
    }

    /// Get the unique identifier for this node (floor, position tuple)
    pub fn id(&self) -> (u32, u32) {
        (self.floor, self.position)
    }
}