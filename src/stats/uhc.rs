use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UHC {
    coins: Option<usize>,
    clearup_achievement: Option<bool>,
    saved_stats: Option<bool>,
    score: Option<usize>,
    kills_solo: Option<usize>,
    deaths_solo: Option<usize>,
}