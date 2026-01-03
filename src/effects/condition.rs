use serde::{Serialize, Deserialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Condition {
    // Target conditions
    TargetIsVulnerable,

    // Hand conditions
    HandAllAttacks,
    HandNoAttacks,

    // Enemy state conditions
    EnemyIsAttacking,

    // Universal conditions
    True,
    False,
}
