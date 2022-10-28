use crate::board::CellShape;
use serde::{Deserialize, Serialize};

/// A struct representing the app configuration, meant to be saved and loaded between sessions.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    /// Whether the player should make the first move.
    pub(crate) player_plays_first: bool,

    /// Which shape the player uses.
    pub(crate) player_shape: CellShape,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player_plays_first: true,
            player_shape: CellShape::X,
        }
    }
}
