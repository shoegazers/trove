use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Profile {
    pub profile_id: Option<String>,
    pub cute_name: Option<String>
}   

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SkyblockBase {
    pub profiles: Option<HashMap<String, Profile>>
}