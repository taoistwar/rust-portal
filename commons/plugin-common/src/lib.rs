use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(FromBytes, ToBytes, Deserialize, PartialEq, Debug, Serialize)]
#[encoding(Json)]
pub struct PluginArgs {
    pub date: String,
    pub args: String,
}
impl PluginArgs {
    pub fn new(date: String, args: String) -> Self {
        Self { date, args }
    }
}

#[cfg(test)]
mod test {}
