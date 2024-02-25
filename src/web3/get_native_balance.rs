use web3::types::{ H160 };
use web3::transports::Http;
use web3::Web3;

use std::str::FromStr;

use crate::validates::env::{ get_rpc };

pub async fn get_balance(address: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Initialize the web3 provider
    let http = Http::new(&get_rpc().unwrap()).unwrap();
    let web3 = Web3::new(http);

    // Convert the address string to an Ethereum Address type
    let address = H160::from_str(address).unwrap();

    // Get the balance of the address
    let balance = web3.eth().balance(address, None).await?;

    // Convert the balance to a string for easier handling
    let balance_str = balance.to_string();

    Ok(balance_str)
}
