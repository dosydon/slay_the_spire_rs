use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlobalInfo {
    pub ascention: u32,
    pub current_floor: u32,
}
