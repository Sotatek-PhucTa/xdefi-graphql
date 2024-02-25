use reqwest::{ Error, header, Client };
use serde_derive::{ Deserialize, Serialize };
use std::env;
use crate::validates::env::{ get_moralis_key };

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub token_address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub balance: String,
}

impl TokenData {
    pub async fn get(user: &String) -> Result<Vec<Self>, Error> {
        let url = format!("https://deep-index.moralis.io/api/v2.2/{}/erc20?chain=bsc", user);

        // Specify your API key
        let api_key = &get_moralis_key().unwrap();

        // Create a new reqwest client
        let client = Client::new();

        // Build the request with the appropriate headers
        let response = client
            .get(url)
            .header("accept", "application/json")
            .header("X-API-Key", api_key)
            .send().await?;

        let mut body: Vec<TokenData> = Vec::new();

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            // Read the response body as a string
            body = response.json().await?;
            // Print the response body
        } else {
            // Print an error message if the request failed
            println!("Request failed with status code: {}", response.status());
        }

        // Ok(res)
        Ok(body)
    }
}
