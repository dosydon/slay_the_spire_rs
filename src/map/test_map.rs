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

    map
}

pub fn test_map_large() -> Map {
    let mut map = Map::new();
    map.add_node(MapNode::new(0, 0, NodeType::Start));

    // First floor nodes
    map.add_node(MapNode::new(1, 0, NodeType::Combat));
    map.add_node(MapNode::new(1, 1, NodeType::Event));
    map.add_node(MapNode::new(1, 3, NodeType::Combat));
    map.add_node(MapNode::new(1, 4, NodeType::Combat));
    map.add_node(MapNode::new(1, 6, NodeType::Combat));

    map.add_edge((0, 0), (1, 0)).unwrap();
    map.add_edge((0, 0), (1, 1)).unwrap();
    map.add_edge((0, 0), (1, 3)).unwrap();
    map.add_edge((0, 0), (1, 4)).unwrap();
    map.add_edge((0, 0), (1, 6)).unwrap();

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

    map
}