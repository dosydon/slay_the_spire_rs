/// Errors that can occur during map operations
#[derive(Debug, Clone, PartialEq)]
pub enum MapError {
    /// The specified node ID doesn't exist
    InvalidNode,
}