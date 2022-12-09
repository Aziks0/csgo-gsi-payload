use serde::Deserialize;

use super::custom::coordinates::Coordinates;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "state")]
pub enum Bomb {
    Carried {
        position: Coordinates,
        player: String, // a SteamID64
    },
    Dropped {
        position: Coordinates,
    },
    Planting {
        position: Coordinates,
        player: String,    // a SteamID64
        countdown: String, // FIXME: float doesn't work
    },
    Planted {
        position: Coordinates,
        countdown: String, // FIXME: float doesn't work
    },
    Defusing {
        position: Coordinates,
        player: String,    // a SteamID64
        countdown: String, // FIXME: float doesn't work
    },
    Defused {
        position: Coordinates,
    },
    Exploded {
        position: Coordinates,
    },
}
