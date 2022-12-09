use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Provider {
    pub name: String, // should always be `Counter-Strike: Global Offensive`
    #[serde(rename = "appid")]
    pub app_id: u16, // should always be `730`
    pub version: u32,
    #[serde(rename = "steamid")]
    pub steam_id: String,
    pub timestamp: u64, // unix time
}
