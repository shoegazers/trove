use serde::Deserialize;
use reqwest::Client;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub const API_URL: &str = "https://api.hypixel.net/v2";

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Stats {

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

pub async fn get_stats(r: HypixelRequest, client: reqwest::Client) -> Result<Player, Box<dyn std::error::Error>> {
    let response = client
            .get(format!("{}/player?uuid={}", API_URL, r.uuid))
            .header("API-Key", r.api_key)
            .send()
            .await?;

        let request: Request = serde_json::from_str(&response.text().await?).unwrap();

        Ok(request.player)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_stats_test() {
        let client = reqwest::Client::new();
        let r = HypixelRequest {
            uuid: "c17dec3e-6675-49e1-9d9a-70fb4be73a06".to_string(),
            api_key: dotenv::var("API_KEY").unwrap()
        };

        let player = get_stats(r, client).await;
        println!("{:#?}", &player);
        assert!(player.is_ok());
    }
}