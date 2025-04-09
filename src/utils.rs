use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct IdSequence(#[serde(deserialize_with = "from_querystring_seq")] Vec<u32>);

impl IdSequence {
    pub fn ids(self) -> Vec<i32> {
        self.0.into_iter().map(|id| id as i32).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn from_querystring_seq<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <String>::deserialize(deserializer)?;

    s.split(',')
        .map(|i| u32::from_str(i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)
}
