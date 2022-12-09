use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum Team {
    T,
    CT,
}
