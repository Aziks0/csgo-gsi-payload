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

/// Information about the state of an individual player.
#[derive(Deserialize, Debug, Clone)]
pub struct PlayerState {
    /// Player health.
    ///
    /// Value range: `0–32767`.
    pub health: u16,
    /// Player armor.
    ///
    /// Value range: `0–100`
    pub armor: u8,
    /// Does the player has an helmet?
    ///
    /// Value is `true` if the player has an helmet, `false` otherwise.
    pub helmet: bool,
    /// How flashed is the player?
    ///
    /// Value range: `0–255`.
    pub flashed: u8,
    /// Is the player burning? (i.e. in a molotov/incendiary)
    ///
    /// Value is `255` if the playing is burning, it ranges `0–254` otherwise.
    pub burning: u8,
    /// How much is the player vision obstructed by a smoke?
    ///
    /// Value range: `0–255`.
    #[serde(default = "default_smoked")] // this field doesn't exist sometimes 🤡
    pub smoked: u8,
    /// Current money of the player.
    ///
    /// Value range: `0–65535`.
    pub money: u16,
    /// Number of kills of the player in the current round.
    pub round_kills: u8,
    /// Number of kills with headshots of the player in the current round.
    #[serde(rename = "round_killhs")]
    pub round_kills_hs: u8,
    /// Total equipment value held by the player.
    pub equip_value: u16, // not sure what the max is, but obviously below 65535
    #[serde(default = "default_false")] // this field doesn't exist when it is false 🤡
    /// Does the player has a defuse kit?
    ///
    /// Value is `true` if the player has a defuse kit, `false` otherwise.
    #[serde(rename = "defusekit")]
    pub defuse_kit: bool,
    #[serde(rename = "round_totaldmg")]
    /// Total number of damages done by the player in the current round.
    pub round_total_damage: Option<u16>,
}

/// Statistics of an individual player.
#[derive(Deserialize, Debug, Clone)]
pub struct MatchStats {
    /// Total number of kills of the player since the start of the game.
    pub kills: i32, // can be a negative number if TK/disconnection
    /// Total number of assists of the player since the start of the game.
    pub assists: u16,
    /// Total number of deaths of the player since the start of the game.
    pub deaths: u16,
    /// Total number of MVPs of the player since the start of the game.
    pub mvps: u16,
    /// Score of the player since the start of the game.
    pub score: u16,
}

/// Activity of a player.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    Playing,
    Menu,
    TextInput,
}

/// Aggregate of information about an individual player.
#[derive(Deserialize, Debug, Clone)]
pub struct Player {
    // `player_id` component
    /// SteamID64 of the player.
    #[serde(rename = "steamid")]
    pub steam_id: Option<String>,
    /// Clan tag of the player.
    pub clan: Option<String>,
    /// Steam name of the player.
    pub name: Option<String>,
    /// Observer slot number of the player.
    pub observer_slot: Option<u16>, // not sure what the max is
    /// Team of the player. See [`Team`].
    pub team: Option<Team>,
    /// Activity of the player. See [`Activity`].
    pub activity: Option<Activity>,

    // `player_state` component
    /// Information about the state of the player. See [`PlayerState`].
    pub state: Option<PlayerState>,

    // `player_position` component
    /// SteamID64 of the player.
    #[serde(rename = "spectarget")]
    pub spec_target: Option<String>,
    /// Position of the player.
    pub position: Option<Coordinates>,
    /// Forward movement of the player.
    pub forward: Option<Coordinates>,

    // `player_match_stats` component
    /// Statistics of the player. See [`MatchStats`].
    pub match_stats: Option<MatchStats>,

    // `player_weapons` component
    /// Weapons carried by the player. See [`Weapon`].
    #[serde(default)]
    pub weapons: HashMap<String, Weapon>, // QUESTION: `Option` instead of HashMap::Default() ?
}
