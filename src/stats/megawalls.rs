#[derive(Deserialize, Debug, Clone, Default)]
pub struct MegaWalls {
    coins: Option<usize>,
    classes: Option<HashMap<String, MegaWallsClass>>,
    packages: Option<Vec<String>>,
    class_points_version: Option<usize>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct MegaWallsClass {
    skill_level_d: Option<usize>,
    checked4: Option<bool>,
    #[serde(rename = "skill_level_dChecked5")] // what the fuck
    skill_level_d_checked5: Option<bool>,
    unlocked: Option<bool>, // this doesnt even show up in some of the response
}