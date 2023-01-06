use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Provider {
    /// Should always be `Counter-Strike: Global Offensive`.
    pub name: String,
    /// Should always be `730`, the steam game id of CSGO.
    #[serde(rename = "appid")]
    pub app_id: u16,
    /// Version of the game.
    pub version: u32,
    /// SteamID64 of the provider.
    #[serde(rename = "steamid")]
    pub steam_id: String,
    /// Unix time.
    pub timestamp: u64,
}
