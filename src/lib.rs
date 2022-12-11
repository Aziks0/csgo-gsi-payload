//! CSGO Game State Integration documentation coming partially from
//! [u/Bkid](reddit_doc) — __[Archive](reddit_doc_archive)__
//!
//! [reddit_doc]: https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/
//! [reddit_doc_archive]: https://web.archive.org/web/20220906050651/https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/

use serde::Deserialize;
use std::collections::HashMap;

mod bomb;
mod custom;
mod grenade;
mod map;
mod phase_countdowns;
mod player;
mod provider;
mod round;
mod team;
mod weapon;

use self::{
    bomb::Bomb, map::Map, phase_countdowns::PhaseCountdowns, player::Player, provider::Provider,
    round::Round,
};
use grenade::Grenade;

#[derive(Deserialize, Debug, Clone)]
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
