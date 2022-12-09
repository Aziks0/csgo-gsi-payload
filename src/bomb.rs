use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use super::custom::coordinates::Coordinates;

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "state")]
pub enum Bomb {
    Carried {
        position: Coordinates,
        /// a SteamID64
        player: String,
    },
    Dropped {
        position: Coordinates,
    },
    Planting {
        position: Coordinates,
        /// a SteamID64
        player: String,
        #[serde_as(as = "DisplayFromStr")]
        countdown: f64,
    },
    Planted {
        position: Coordinates,
        #[serde_as(as = "DisplayFromStr")]
        countdown: f64,
    },
    Defusing {
        position: Coordinates,
        /// a SteamID64
        player: String,
        #[serde_as(as = "DisplayFromStr")]
        countdown: f64,
    },
    Defused {
        position: Coordinates,
    },
    Exploded {
        position: Coordinates,
    },
}
