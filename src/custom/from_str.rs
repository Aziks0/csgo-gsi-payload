//! Source from [dtolnay](from_str_src)
//!
//! [from_str_src]: https://github.com/serde-rs/json/issues/317#issuecomment-300251188

use serde::de::{self, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
