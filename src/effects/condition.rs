#[derive(Copy, Debug, Clone, PartialEq)]
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
