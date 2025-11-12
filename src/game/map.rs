//! Simple static map representation using graph structure

use std::collections::{HashMap, HashSet};

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
    /// Unique identifier for this node
    pub id: u32,
    /// Floor level (0-based)
    pub floor: u32,
    /// Position on the floor (0-based, left to right)
    pub position: u32,
    /// Type of encounter at this node
    pub node_type: NodeType,
}

impl MapNode {
    pub fn new(id: u32, floor: u32, position: u32, node_type: NodeType) -> Self {
        MapNode {
            id,
            floor,
            position,
            node_type,
        }
    }
}

/// Static graph-based map representation
#[derive(Debug, Clone)]
pub struct Map {
    /// All nodes in the map, indexed by node ID
    nodes: HashMap<u32, MapNode>,
    /// Adjacency list representation: node_id -> set of connected node_ids
    adjacency_list: HashMap<u32, HashSet<u32>>,
}

impl Map {
    /// Create a new empty map
    pub fn new() -> Self {
        Map {
            nodes: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }
    
    /// Add a node to the map
    pub fn add_node(&mut self, node: MapNode) {
        let node_id = node.id;
        self.nodes.insert(node_id, node);
        self.adjacency_list.insert(node_id, HashSet::new());
    }
    
    /// Add a directed edge from one node to another
    pub fn add_edge(&mut self, from_node: u32, to_node: u32) -> Result<(), MapError> {
        // Verify both nodes exist
        if !self.nodes.contains_key(&from_node) || !self.nodes.contains_key(&to_node) {
            return Err(MapError::InvalidNode);
        }
        
        self.adjacency_list.entry(from_node)
            .or_insert_with(HashSet::new)
            .insert(to_node);
        
        Ok(())
    }
    
    /// Get all neighbors of a specific node
    pub fn get_neighbors(&self, node_id: u32) -> Vec<u32> {
        self.adjacency_list.get(&node_id)
            .map(|neighbors| neighbors.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Check if there's a direct path from one node to another
    pub fn has_edge(&self, from_node: u32, to_node: u32) -> bool {
        self.adjacency_list.get(&from_node)
            .map(|neighbors| neighbors.contains(&to_node))
            .unwrap_or(false)
    }
    
    /// Get a node by ID
    pub fn get_node(&self, node_id: u32) -> Option<&MapNode> {
        self.nodes.get(&node_id)
    }
    
    /// Get all nodes
    pub fn get_all_nodes(&self) -> Vec<&MapNode> {
        let mut nodes: Vec<&MapNode> = self.nodes.values().collect();
        nodes.sort_by_key(|node| (node.floor, node.position));
        nodes
    }
    
    /// Get the adjacency list
    pub fn get_adjacency_list(&self) -> &HashMap<u32, HashSet<u32>> {
        &self.adjacency_list
    }
}

/// Errors that can occur during map operations
#[derive(Debug, Clone, PartialEq)]
pub enum MapError {
    /// The specified node ID doesn't exist
    InvalidNode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_map_creation() {
        let map = Map::new();
        assert_eq!(map.nodes.len(), 0);
        assert_eq!(map.adjacency_list.len(), 0);
    }
    
    #[test]
    fn test_add_nodes() {
        let mut map = Map::new();
        
        let node1 = MapNode::new(1, 0, 0, NodeType::Start);
        let node2 = MapNode::new(2, 1, 0, NodeType::Combat);
        
        map.add_node(node1);
        map.add_node(node2);
        
        assert_eq!(map.nodes.len(), 2);
        assert!(map.nodes.contains_key(&1));
        assert!(map.nodes.contains_key(&2));
    }
    
    #[test]
    fn test_add_edges() {
        let mut map = Map::new();
        
        map.add_node(MapNode::new(1, 0, 0, NodeType::Start));
        map.add_node(MapNode::new(2, 1, 0, NodeType::Combat));
        
        let result = map.add_edge(1, 2);
        assert!(result.is_ok());
        assert!(map.has_edge(1, 2));
        assert!(!map.has_edge(2, 1)); // Directed graph
    }
    
    #[test]
    fn test_get_neighbors() {
        let mut map = Map::new();
        
        map.add_node(MapNode::new(1, 0, 0, NodeType::Start));
        map.add_node(MapNode::new(2, 1, 0, NodeType::Combat));
        map.add_node(MapNode::new(3, 1, 1, NodeType::Elite));
        map.add_edge(1, 2).unwrap();
        map.add_edge(1, 3).unwrap();
        
        let neighbors = map.get_neighbors(1);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
    }
    
    #[test]
    fn test_get_node() {
        let mut map = Map::new();
        let node = MapNode::new(1, 0, 0, NodeType::Start);
        map.add_node(node);
        
        let retrieved = map.get_node(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().node_type, NodeType::Start);
        
        let non_existent = map.get_node(999);
        assert!(non_existent.is_none());
    }
    
    #[test]
    fn test_invalid_edge() {
        let mut map = Map::new();
        map.add_node(MapNode::new(1, 0, 0, NodeType::Start));
        
        // Try to add edge to non-existent node
        let result = map.add_edge(1, 999);
        assert_eq!(result, Err(MapError::InvalidNode));
    }
}