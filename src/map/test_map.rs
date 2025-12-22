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
    map.add_node(MapNode::new(1, 0, NodeType::Event));
    map.add_node(MapNode::new(1, 1, NodeType::Combat));
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
    map.add_edge((1, 1), (2, 1)).unwrap();
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

    map.add_node(MapNode::new(9, 0, NodeType::Treasure));
    map.add_node(MapNode::new(9, 1, NodeType::Treasure));
    map.add_node(MapNode::new(9, 3, NodeType::Treasure));
    map.add_node(MapNode::new(9, 4, NodeType::Treasure));

    map.add_edge((8, 0), (9, 0)).unwrap();
    map.add_edge((8, 1), (9, 1)).unwrap();
    map.add_edge((8, 2), (9, 3)).unwrap();
    map.add_edge((8, 4), (9, 3)).unwrap();
    map.add_edge((8, 5), (9, 4)).unwrap();

    map.add_node(MapNode::new(10, 0, NodeType::RestSite));
    map.add_node(MapNode::new(10, 2, NodeType::Event));
    map.add_node(MapNode::new(10, 4, NodeType::Combat));

    map.add_edge((9, 0), (10, 0)).unwrap();
    map.add_edge((9, 1), (10, 0)).unwrap();
    map.add_edge((9, 1), (10, 2)).unwrap();
    map.add_edge((9, 3), (10, 4)).unwrap();
    map.add_edge((9, 4), (10, 4)).unwrap();

    map.add_node(MapNode::new(11, 0, NodeType::Elite));
    map.add_node(MapNode::new(11, 1, NodeType::Event));
    map.add_node(MapNode::new(11, 2, NodeType::Event));
    map.add_node(MapNode::new(11, 3, NodeType::Combat));
    map.add_node(MapNode::new(11, 4, NodeType::Shop));
    map.add_node(MapNode::new(11, 5, NodeType::Elite));

    map.add_edge((10, 0), (11, 0)).unwrap();
    map.add_edge((10, 0), (11, 1)).unwrap();
    map.add_edge((10, 2), (11, 2)).unwrap();
    map.add_edge((10, 4), (11, 3)).unwrap();
    map.add_edge((10, 4), (11, 4)).unwrap();
    map.add_edge((10, 4), (11, 5)).unwrap();

    map.add_node(MapNode::new(12, 0, NodeType::Combat));
    map.add_node(MapNode::new(12, 2, NodeType::RestSite));
    map.add_node(MapNode::new(12, 3, NodeType::RestSite));
    map.add_node(MapNode::new(12, 5, NodeType::Event));

    map.add_edge((11, 0), (12, 0)).unwrap();
    map.add_edge((11, 1), (12, 2)).unwrap();
    map.add_edge((11, 2), (12, 2)).unwrap();
    map.add_edge((11, 3), (12, 2)).unwrap();
    map.add_edge((11, 4), (12, 3)).unwrap();
    map.add_edge((11, 5), (12, 5)).unwrap();

    map.add_node(MapNode::new(13, 1, NodeType::Combat));
    map.add_node(MapNode::new(13, 2, NodeType::Event));
    map.add_node(MapNode::new(13, 5, NodeType::Shop));

    map.add_edge((12, 0), (13, 1)).unwrap();
    map.add_edge((12, 2), (13, 1)).unwrap();
    map.add_edge((12, 2), (13, 2)).unwrap();
    map.add_edge((12, 3), (13, 2)).unwrap();
    map.add_edge((12, 5), (13, 5)).unwrap();

    map.add_node(MapNode::new(14, 1, NodeType::Combat));
    map.add_node(MapNode::new(14, 2, NodeType::Event));
    map.add_node(MapNode::new(14, 3, NodeType::Elite));
    map.add_node(MapNode::new(14, 4, NodeType::Combat));

    map.add_edge((13, 1), (14, 1)).unwrap();
    map.add_edge((13, 2), (14, 2)).unwrap();
    map.add_edge((13, 2), (14, 3)).unwrap();
    map.add_edge((13, 5), (14, 4)).unwrap();

    map.add_node(MapNode::new(15, 1, NodeType::RestSite));
    map.add_node(MapNode::new(15, 3, NodeType::RestSite));

    map.add_edge((14, 1), (15, 1)).unwrap();
    map.add_edge((14, 2), (15, 1)).unwrap();
    map.add_edge((14, 3), (15, 3)).unwrap();
    map.add_edge((14, 4), (15, 3)).unwrap();

    map.add_node(MapNode::new(16, 3, NodeType::Boss));

    map.add_edge((15, 1), (16, 3)).unwrap();
    map.add_edge((15, 3), (16, 3)).unwrap();

    // Set starting position
    map.set_starting_position((0, 3)).unwrap();

    map
}