use serde::Deserialize;
use std::collections::HashMap;

use super::custom::coordinates::Coordinates;
use super::custom::default::default_false;
use super::team::Team;
use super::weapon::Weapon;

#[inline]
fn default_smoked() -> u8 {
    0
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerState {
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
    #[serde(default = "default_false")] // this field doesn't exist when it is false 🤡
    #[serde(rename = "defusekit")]
    pub defuse_kit: bool,
    #[serde(rename = "round_totaldmg")]
    pub round_total_damage: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MatchStats {
    pub kills: i32, // can be a negative number if TK/disconnection
    pub assists: u16,
    pub deaths: u16,
    pub mvps: u16,
    pub score: u16,
}

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
    pub state: Option<PlayerState>,

    // `player_position` component
    #[serde(rename = "spectarget")]
    pub spec_target: Option<String>, // a SteamID64
    pub position: Option<Coordinates>,
    pub forward: Option<Coordinates>,

    // `player_match_stats` component
    pub match_stats: Option<MatchStats>,

    // `player_weapons` component
    #[serde(default)]
    pub weapons: HashMap<String, Weapon>, // QUESTION: `Option` instead of HashMap::Default() ?
}
