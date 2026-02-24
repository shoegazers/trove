use serde::Deserialize;
use reqwest::Client;
pub mod utils;
pub mod stats;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub const API_URL: &str = "https://api.hypixel.net/v2";

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Stats {
    #[serde(rename = "SkyBlock")] // why??
    pub skyblock: Option<stats::skyblock::SkyblockBase>,
}   

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Player { 
    pub uuid: Option<String>,
    pub displayname: Option<String>,
    pub rank: Option<String>,
    #[serde(rename = "packageRank")]
    pub package_rank: Option<String>,
    #[serde(rename = "newPackageRank")]
    pub new_package_rank: Option<String>,
    #[serde(rename = "monthlyPackageRank")]
    pub monthly_package_rank: Option<String>,
    #[serde(rename = "firstLogin")]
    pub first_login: Option<usize>,
    #[serde(rename = "lastLogin")]
    pub last_login: Option<usize>,
    #[serde(rename = "lastLogout")]
    pub last_logout: Option<usize>,
    #[serde(rename = "achievementsOneTime")]
    pub one_time_achievements: Option<Vec<String>>,
    pub stats: Option<Stats>
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Request {
    pub success: bool,
    pub player: Player
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct HypixelRequest {
    pub uuid: String,
    pub api_key: String,
}

pub async fn make_request(r: HypixelRequest, client: Client) -> Result<Request, Box<dyn std::error::Error>> {
    let response = client
            .get(format!("{}/player?uuid={}", API_URL, r.uuid))
            .header("API-Key", r.api_key)
            .send()
            .await?;

        let request: Request = serde_json::from_str(&response.text().await?).unwrap();

        Ok(request)
}

pub async fn get_stats(r: HypixelRequest) -> Result<Player, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = make_request(r, client).await?;

    Ok(request.player)
}

pub async fn get_achievements(r: HypixelRequest) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = make_request(r, client).await?;

    Ok(request.player.one_time_achievements.unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_stats_test() {
        let r = HypixelRequest {
            uuid: "c17dec3e-6675-49e1-9d9a-70fb4be73a06".to_string(),
            api_key: dotenv::var("API_KEY").unwrap()
        };

        let player = get_stats(r).await;
        println!("{:#?}", &player);
        assert!(player.is_ok());
    }

    #[tokio::test]
    async fn get_skyblock_test() {
        let r = HypixelRequest {
            uuid: "c17dec3e-6675-49e1-9d9a-70fb4be73a06".to_string(),
            api_key: dotenv::var("API_KEY").unwrap()
        };
        let p = get_stats(r).await;
        println!("{:#?}", &p.unwrap_or_default().stats.unwrap_or_default().skyblock);
    }
}