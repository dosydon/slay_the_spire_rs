use std::collections::HashMap;
use crate::map::node::{MapNode, NodeType};
use crate::map::error::MapError;

/// Static graph-based map representation
#[derive(Debug, Clone)]
pub struct Map {
    /// All nodes in the map, indexed by (floor, position)
    nodes: HashMap<(u32, u32), MapNode>,
    /// Adjacency list representation: (floor, position) -> list of connected (floor, position)
    /// Order is preserved to maintain consistent path choices
    adjacency_list: HashMap<(u32, u32), Vec<(u32, u32)>>,
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
        let node_id = node.id();
        self.nodes.insert(node_id, node);
        self.adjacency_list.insert(node_id, Vec::new());
    }

    /// Add a directed edge from one node to another
    pub fn add_edge(&mut self, from_node: (u32, u32), to_node: (u32, u32)) -> Result<(), MapError> {
        // Verify both nodes exist
        if !self.nodes.contains_key(&from_node) || !self.nodes.contains_key(&to_node) {
            return Err(MapError::InvalidNode);
        }

        // Add edge only if it doesn't already exist
        let neighbors = self.adjacency_list.entry(from_node)
            .or_insert_with(Vec::new);

        if !neighbors.contains(&to_node) {
            neighbors.push(to_node);
        }

        Ok(())
    }

    /// Get all neighbors of a specific node
    /// Returns the neighbors in the order they were added (preserves path choice ordering)
    pub fn get_neighbors(&self, node_id: (u32, u32)) -> Vec<(u32, u32)> {
        self.adjacency_list.get(&node_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if there's a direct path from one node to another
    pub fn has_edge(&self, from_node: (u32, u32), to_node: (u32, u32)) -> bool {
        self.adjacency_list.get(&from_node)
            .map(|neighbors| neighbors.contains(&to_node))
            .unwrap_or(false)
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: (u32, u32)) -> Option<&MapNode> {
        self.nodes.get(&node_id)
    }

    /// Get all nodes
    pub fn get_all_nodes(&self) -> Vec<&MapNode> {
        let mut nodes: Vec<&MapNode> = self.nodes.values().collect();
        nodes.sort_by_key(|node| (node.floor, node.position));
        nodes
    }

    /// Get the adjacency list
    pub fn get_adjacency_list(&self) -> &HashMap<(u32, u32), Vec<(u32, u32)>> {
        &self.adjacency_list
    }
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

        let node1 = MapNode::new(0, 0, NodeType::Start);
        let node2 = MapNode::new(1, 0, NodeType::Combat);

        map.add_node(node1);
        map.add_node(node2);

        assert_eq!(map.nodes.len(), 2);
        assert!(map.nodes.contains_key(&(0, 0)));
        assert!(map.nodes.contains_key(&(1, 0)));
    }

    #[test]
    fn test_add_edges() {
        let mut map = Map::new();

        map.add_node(MapNode::new(0, 0, NodeType::Start));
        map.add_node(MapNode::new(1, 0, NodeType::Combat));

        let result = map.add_edge((0, 0), (1, 0));
        assert!(result.is_ok());
        assert!(map.has_edge((0, 0), (1, 0)));
        assert!(!map.has_edge((1, 0), (0, 0))); // Directed graph
    }

    #[test]
    fn test_get_neighbors() {
        let mut map = Map::new();

        map.add_node(MapNode::new(0, 0, NodeType::Start));
        map.add_node(MapNode::new(1, 0, NodeType::Combat));
        map.add_node(MapNode::new(1, 1, NodeType::Elite));
        map.add_edge((0, 0), (1, 0)).unwrap();
        map.add_edge((0, 0), (1, 1)).unwrap();

        let neighbors = map.get_neighbors((0, 0));
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 1)));
    }

    #[test]
    fn test_get_node() {
        let mut map = Map::new();
        let node = MapNode::new(0, 0, NodeType::Start);
        map.add_node(node);

        let retrieved = map.get_node((0, 0));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().node_type, NodeType::Start);

        let non_existent = map.get_node((9, 9));
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_invalid_edge() {
        let mut map = Map::new();
        map.add_node(MapNode::new(0, 0, NodeType::Start));

        // Try to add edge to non-existent node
        let result = map.add_edge((0, 0), (9, 9));
        assert_eq!(result, Err(MapError::InvalidNode));
    }
}