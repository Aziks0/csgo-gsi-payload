use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use super::custom::coordinates::Coordinates;

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "state")]
pub enum Bomb {
    Carried {
        /// Position of the player carrying the bomb.
        position: Coordinates,
        /// SteamID64 of the player carrying the bomb.
        player: String,
    },
    Dropped {
        /// Position of the dropped bomb.
        position: Coordinates,
    },
    Planting {
        /// Position of the player currently planting the bomb.
        position: Coordinates,
        /// SteamID64 of the player currently planting the bomb.
        player: String,
        /// Time left before the state of the bomb changes to [`Planted`](`Bomb::Planted`)
        #[serde_as(as = "DisplayFromStr")]
        countdown: f64,
    },
    Planted {
        /// Position of the planted bomb.
        position: Coordinates,
        /// Time left before the state of the bomb changes to [`Exploded`](`Bomb::Exploded`)
        #[serde_as(as = "DisplayFromStr")]
        countdown: f64,
    },
    Defusing {
        /// Position of the player defusing the bomb.
        position: Coordinates,
        /// SteamID64 of the player defusing the bomb.
        player: String,
        #[serde_as(as = "DisplayFromStr")]
        /// Time left before the bomb state changes to [`Defused`](`Bomb::Defused`).
        countdown: f64,
    },
    Defused {
        /// Position of the defused bomb.
        position: Coordinates,
    },
    Exploded {
        /// Position of the exploded bomb.
        position: Coordinates,
    },
}
