use crate::map::graph::Map;
use crate::map::node::{MapNode, NodeType};

pub fn debug_map() -> Map {
    let mut map = Map::new();
    map.add_node(MapNode::new(0, 0, NodeType::Start));

    // First floor nodes
    map.add_node(MapNode::new(1, 0, NodeType::Combat));
    map.add_node(MapNode::new(1, 1, NodeType::Event));
    map.add_node(MapNode::new(1, 3, NodeType::Elite));
    map.add_node(MapNode::new(1, 4, NodeType::RestSite));
    map.add_node(MapNode::new(1, 5, NodeType::Shop));

    map.add_edge((0, 0), (1, 0)).unwrap();
    map.add_edge((0, 0), (1, 1)).unwrap();
    map.add_edge((0, 0), (1, 3)).unwrap();
    map.add_edge((0, 0), (1, 4)).unwrap();
    map.add_edge((0, 0), (1, 5)).unwrap();

    // Set starting position
    map.set_starting_position((0, 0)).unwrap();

    map
}

pub fn test_map_large() -> Map {
    let mut map = Map::new();
    map.add_node(MapNode::new(0, 3, NodeType::Start));

    // First floor nodes
    map.add_node(MapNode::new(1, 0, NodeType::Combat));
    map.add_node(MapNode::new(1, 1, NodeType::Event));
    map.add_node(MapNode::new(1, 3, NodeType::Combat));
    map.add_node(MapNode::new(1, 4, NodeType::Combat));
    map.add_node(MapNode::new(1, 6, NodeType::Combat));

    map.add_edge((0, 3), (1, 0)).unwrap();
    map.add_edge((0, 3), (1, 1)).unwrap();
    map.add_edge((0, 3), (1, 3)).unwrap();
    map.add_edge((0, 3), (1, 4)).unwrap();
    map.add_edge((0, 3), (1, 6)).unwrap();

    // Second floor nodes
    map.add_node(MapNode::new(2, 0, NodeType::Event));
    map.add_node(MapNode::new(2, 1, NodeType::Combat));
    map.add_node(MapNode::new(2, 3, NodeType::Combat));
    map.add_node(MapNode::new(2, 4, NodeType::Combat));
    map.add_node(MapNode::new(2, 6, NodeType::Combat));

    map.add_edge((1, 0), (2, 0)).unwrap();
    map.add_edge((1, 3), (2, 1)).unwrap();
    map.add_edge((1, 3), (2, 3)).unwrap();
    map.add_edge((1, 4), (2, 4)).unwrap();
    map.add_edge((1, 6), (2, 6)).unwrap();

    // Third floor nodes
    map.add_node(MapNode::new(3, 0, NodeType::Event));
    map.add_node(MapNode::new(3, 1, NodeType::Combat));
    map.add_node(MapNode::new(3, 2, NodeType::Combat));
    map.add_node(MapNode::new(3, 5, NodeType::Combat));
    map.add_node(MapNode::new(3, 6, NodeType::Combat));

    map.add_edge((2, 0), (3, 0)).unwrap();
    map.add_edge((2, 0), (3, 1)).unwrap();
    map.add_edge((2, 1), (3, 1)).unwrap();
    map.add_edge((2, 3), (3, 2)).unwrap();
    map.add_edge((2, 4), (3, 5)).unwrap();
    map.add_edge((2, 6), (3, 6)).unwrap();

    // Fourth floor nodes
    map.add_node(MapNode::new(4, 1, NodeType::Event));
    map.add_node(MapNode::new(4, 2, NodeType::Shop));
    map.add_node(MapNode::new(4, 4, NodeType::Combat));
    map.add_node(MapNode::new(4, 6, NodeType::Combat));

    map.add_edge((3, 0), (4, 1)).unwrap();
    map.add_edge((3, 1), (4, 2)).unwrap();
    map.add_edge((3, 2), (4, 2)).unwrap();
    map.add_edge((3, 5), (4, 4)).unwrap();
    map.add_edge((3, 6), (4, 6)).unwrap();

    // Fifth floor nodes
    map.add_node(MapNode::new(5, 1, NodeType::Combat));
    map.add_node(MapNode::new(5, 5, NodeType::Event));

    map.add_edge((4, 1), (5, 1)).unwrap();
    map.add_edge((4, 2), (5, 1)).unwrap();
    map.add_edge((4, 4), (5, 5)).unwrap();
    map.add_edge((4, 6), (5, 5)).unwrap();

    // Sixth floor nodes
    map.add_node(MapNode::new(6, 0, NodeType::Combat));
    map.add_node(MapNode::new(6, 1, NodeType::Elite));
    map.add_node(MapNode::new(6, 4, NodeType::Elite));
    map.add_node(MapNode::new(6, 6, NodeType::RestSite));

    map.add_edge((5, 1), (6, 0)).unwrap();
    map.add_edge((5, 1), (6, 1)).unwrap();
    map.add_edge((5, 5), (6, 4)).unwrap();
    map.add_edge((5, 5), (6, 6)).unwrap();

    // Seventh floor nodes
    map.add_node(MapNode::new(7, 0, NodeType::Elite));
    map.add_node(MapNode::new(7, 1, NodeType::Combat));
    map.add_node(MapNode::new(7, 2, NodeType::RestSite));
    map.add_node(MapNode::new(7, 3, NodeType::Combat));
    map.add_node(MapNode::new(7, 6, NodeType::Event));

    map.add_edge((6, 0), (7, 0)).unwrap();
    map.add_edge((6, 1), (7, 1)).unwrap();
    map.add_edge((6, 1), (7, 2)).unwrap();
    map.add_edge((6, 4), (7, 3)).unwrap();
    map.add_edge((6, 6), (7, 6)).unwrap();

    // Eighth floor nodes
    map.add_node(MapNode::new(8, 0, NodeType::RestSite));
    map.add_node(MapNode::new(8, 1, NodeType::Elite));
    map.add_node(MapNode::new(8, 2, NodeType::Event));
    map.add_node(MapNode::new(8, 4, NodeType::Combat));
    map.add_node(MapNode::new(8, 5, NodeType::RestSite));

    map.add_edge((7, 0), (8, 0)).unwrap();
    map.add_edge((7, 1), (8, 0)).unwrap();
    map.add_edge((7, 2), (8, 1)).unwrap();
    map.add_edge((7, 2), (8, 2)).unwrap();
    map.add_edge((7, 3), (8, 4)).unwrap();
    map.add_edge((7, 6), (8, 5)).unwrap();

    // Set starting position
    map.set_starting_position((0, 3)).unwrap();

    map
}