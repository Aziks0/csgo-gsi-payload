use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Over,
    Live,
    Bomb,
    Defuse,
    Warmup,
    FreezeTime,
    #[serde(rename = "timeout_t")]
    TimeoutT,
    #[serde(rename = "timeout_ct")]
    TimeoutCT,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhaseCountdowns {
    pub phase: Phase,
    pub phase_ends_in: String, // FIXME: float doesn't work // not sure what the max is
}
