use color_eyre::eyre::eyre;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,

    pub addr: Option<IpAddr>,
    pub port: Option<u16>,
}

impl Config {
    pub fn from_env() -> color_eyre::Result<Self> {
        match envy::from_env::<Self>() {
            Ok(c) => Ok(c),
            Err(err) => match err {
                envy::Error::MissingValue(varname) => Err(eyre!(
                    "missing environment variable: {}",
                    varname.to_uppercase()
                )),
                envy::Error::Custom(msg) => Err(eyre!("cannot deserialize env: {msg}")),
            },
        }
    }
}
