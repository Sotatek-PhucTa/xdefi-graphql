use juniper::{ EmptyMutation, EmptySubscription, FieldResult, RootNode, GraphQLObject };

use crate::validates::is_eth_address::{ is_valid_ethereum_address, is_zero_ethereum_address };
use crate::errors::Error::{ InvalidAddress };
use crate::web3::get_native_balance::get_balance;
use crate::web3::get_token_balance::get_balances;
use crate::web3::get_decimals::get_decimals;

#[derive(GraphQLObject)]
#[graphql(description = "Balance")]
struct Balance {
    asset_address: String,
    value: String,
    decimals: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn getBalance(
        address: String,
        asset_addresses: Vec<String>
    ) -> FieldResult<Vec<Balance>> {
        let mut result: Vec<Balance> = Vec::new();
        // Check if the main address is a valid Ethereum address
        if !is_valid_ethereum_address(&address) {
            return Err(InvalidAddress.into());
        }

        // Check if all asset addresses are valid Ethereum addresses
        let mut assets_without_native: Vec<String> = Vec::new();

        for asset_address in &asset_addresses {
            if !is_valid_ethereum_address(asset_address) {
                return Err(InvalidAddress.into());
            }

            if !is_zero_ethereum_address(asset_address) {
                assets_without_native.push(asset_address.to_string());
            }
        }

        let balances = get_balances(&address, &assets_without_native).await;
        let decimals = get_decimals(&asset_addresses).await;

        if asset_addresses.len() > assets_without_native.len() {
            result.push(Balance {
                asset_address: "0x0000000000000000000000000000000000000000".into(),
                value: get_balance(&address).await?,
                decimals: 18,
            });
        }

        for (i, asset_without_native) in assets_without_native.iter().enumerate() {
            result.push(Balance {
                asset_address: asset_without_native.to_string(),
                value: balances[i].clone(),
                decimals: decimals[i] as i32,
            });
        }

        Ok(result)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
