use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct TeamSubsection {
    pub score: u16, // not sure what the max is
    pub consecutive_round_losses: u16,
    pub timeouts_remaining: u16,
    pub matches_won_this_series: u16,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Live,
    Warmup,
    Intermission,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Competitive,
    Casual,
    Deathmatch,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RoundWinCondition {
    CtWinElimination,
    CtWinDefuse,
    CtWinTime,
    TWinElimination,
    TWinBomb,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Map {
    pub mode: Mode,
    pub name: String,
    pub phase: Phase,
    pub round: u32,
    pub team_t: TeamSubsection,
    pub team_ct: TeamSubsection,
    pub num_matches_to_win_series: u16,
    pub current_spectator: Option<u64>,
    pub souvenirs_total: u64,
    #[serde(default)]
    pub round_wins: HashMap<u16, RoundWinCondition>,
}
