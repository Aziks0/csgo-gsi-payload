use serde::Deserialize;

use super::custom::coordinates::Coordinates;
use super::team::Team;

#[inline]
fn default_smoked() -> u8 {
    0
}

#[derive(Deserialize, Debug, Clone)]
pub struct State {
    pub health: u16, // 0–32767
    pub armor: u8,   // 0–100
    pub helmet: bool,
    pub flashed: u8, // 0–255
    pub burning: u8, // 0–255
    #[serde(default = "default_smoked")] // this field doesn't exist sometimes 🤡
    pub smoked: u8, // 0–255
    pub money: u16,  // 0–65535
    pub round_kills: u8,
    #[serde(rename = "round_killhs")]
    pub round_kills_hs: u8,
    pub equip_value: u16, // not sure what the max is, but obviously below 65535
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchStats {
    pub kills: i32, // can be a negative number if TK/disconnection
    pub assists: u16,
    pub deaths: u16,
    pub mvps: u16,
    pub score: u16,
}

// TODO
#[derive(Deserialize, Debug, Clone)]
pub struct Weapons {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    Playing,
    Menu,
    TextInput,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    // `player_id` component
    #[serde(rename = "steamid")]
    pub steam_id: Option<String>,
    pub clan: Option<String>,
    pub name: Option<String>,
    pub observer_slot: Option<u16>, // not sure what the max is
    pub team: Option<Team>,

    // `player_state` component
    pub state: Option<State>,

    // `player_position` component
    #[serde(rename = "spectarget")]
    pub spec_target: Option<String>, // a SteamID64
    pub position: Option<Coordinates>,
    pub forward: Option<Coordinates>,

    // `player_match_stats` component
    pub match_stats: Option<MatchStats>,

    // `player_weapons` component
    pub weapons: Option<Weapons>,
}
