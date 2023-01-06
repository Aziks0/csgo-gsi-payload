//! Data structures for CSGO Game State Integration payload.
//!
//! CSGO Game State Integration documentation coming partially from
//! [u/Bkid](https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/)
//! — __[Archive](https://web.archive.org/web/20220906050651/https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/)__
//!

use serde::Deserialize;
use std::collections::HashMap;

mod bomb;
mod custom;
mod provider;
mod team;

// no re-export for these because they contain multiple struct/enums
pub mod grenade;
pub mod map;
pub mod phase_countdowns;
pub mod player;
pub mod round;
pub mod weapon;

// re-export to avoid repetition
pub use bomb::Bomb;
pub use custom::coordinates::Coordinates;
pub use provider::Provider;
pub use team::Team;

use self::{
    grenade::Grenade, map::Map, phase_countdowns::PhaseCountdowns, player::Player, round::Round,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Payload {
    pub provider: Option<Provider>,
    pub player: Option<Player>,
    pub bomb: Option<Bomb>,
    pub round: Option<Round>,
    pub phase_countdowns: Option<PhaseCountdowns>,
    pub map: Option<Map>,

    #[serde(rename = "allplayers")]
    pub all_players: Option<HashMap<String, Player>>,
    pub grenades: Option<HashMap<String, Grenade>>,

    pub added: Option<serde_json::Value>,
    pub previously: Option<serde_json::Value>,
}
