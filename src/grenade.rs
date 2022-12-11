use std::collections::HashMap;

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use super::custom::coordinates::Coordinates;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GrenadeType {
    Frag,
    Smoke,
    Decoy,
    Inferno,
    Firebomb,
    Flashbang,
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Grenade {
    /// a SteamID64
    pub owner: String,
    #[serde_as(as = "DisplayFromStr")]
    pub lifetime: f64,
    pub position: Option<Coordinates>,
    pub velocity: Option<Coordinates>,
    pub r#type: GrenadeType,
    #[serde(rename = "effecttime")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub effect_time: Option<f64>,
    pub flames: Option<HashMap<String, Coordinates>>,
}
