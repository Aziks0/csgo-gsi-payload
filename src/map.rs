use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct TeamSubsection {
    /// Score of this team.
    pub score: u16,
    /// Number of consecutive rounds losses of this team.
    pub consecutive_round_losses: u16,
    /// Number of timeouts remaining for this team.
    pub timeouts_remaining: u16,
    /// Number of matches won by this team, in this series.
    ///
    /// Used on _"Best of X"_ games.
    pub matches_won_this_series: u16,
    /// Name of this team.
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Live,
    Warmup,
    Intermission,
    GameOver,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Competitive,
    Casual,
    Deathmatch,
    #[serde(rename = "scrimcomp2v2")]
    Wingman,
    #[serde(rename = "gungameprogressive")]
    GunGame,
    #[serde(rename = "gungametrbomb")]
    Demolition,
    Training,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RoundWinCondition {
    /// When CTs win the round by eliminating all the Ts.
    CtWinElimination,
    /// When CTs win the round by defusing the bomb.
    CtWinDefuse,
    /// When CTs win the round by time.
    CtWinTime,
    /// When Ts win the round by eliminating all the CTs.
    TWinElimination,
    /// When Ts win the round by exploding the bomb.
    TWinBomb,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    /// Gamemode. See [`Mode`].
    pub mode: Mode,
    /// Name of the map.
    pub name: String,
    /// Current phase of the game. See [`Phase`].
    pub phase: Phase,
    /// Current round number.
    pub round: u32,
    /// Information about the Ts. See [`TeamSubsection`].
    pub team_t: TeamSubsection,
    /// Information about the CTs. See [`TeamSubsection`].
    pub team_ct: TeamSubsection,
    /// Number of matches to win the series.
    ///
    /// Used on _"Best of X"_ games.
    pub num_matches_to_win_series: u16,
    /// Number of spectators.
    pub current_spectators: Option<u64>,
    /// Number of souvenirs packages dropped during the game.
    pub souvenirs_total: u64,
    /// See [`RoundWinCondition`].
    #[serde(default)]
    pub round_wins: HashMap<u16, RoundWinCondition>, // QUESTION: `Option` instead of HashMap::Default() ?
}
