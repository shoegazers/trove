use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UHC {
    pub coins: Option<usize>,
    pub clearup_achievement: Option<bool>,
    pub saved_stats: Option<bool>,
    pub score: Option<usize>,
    pub kills_solo: Option<usize>,
    pub deaths_solo: Option<usize>,
}