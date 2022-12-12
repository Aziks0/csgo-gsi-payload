use serde::Deserialize;

use super::team::Team;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    Planted,
    Exploded,
    Defused,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "phase")]
pub enum Round {
    FreezeTime,
    Over { win_team: Option<Team> },
    Live { bomb: Option<BombState> },
}
