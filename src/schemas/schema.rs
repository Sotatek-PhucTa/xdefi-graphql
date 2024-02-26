use std::sync::Mutex;

use juniper::{ EmptyMutation, EmptySubscription, FieldResult, RootNode, GraphQLObject };

use crate::context::Context;
use crate::validates::is_eth_address::{ is_valid_ethereum_address, is_zero_ethereum_address };
use crate::errors::Error::{ InvalidAddress };
use crate::web3::get_native_balance::get_balance;
use crate::web3::get_token_balance::get_balances;
use crate::web3::get_decimals::get_decimals;
use crate::cache::TokenCache;

#[derive(GraphQLObject, Clone)]
#[graphql(description = "Balance")]
pub struct Balance {
    pub asset_address: String,
    pub value: String,
    pub decimals: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn getBalance(
        ctx: &Context,
        address: String,
        asset_addresses: Vec<String>
    ) -> FieldResult<Vec<Balance>> {
        let mut result: Vec<Balance> = Vec::new();
        // Check if the main address is a valid Ethereum address
        if !is_valid_ethereum_address(&address) {
            return Err(InvalidAddress.into());
        }

        result.push(Balance {
            asset_address: "0x0000000000000000000000000000000000000000".into(),
            value: get_balance(&address).await?,
            decimals: 18,
        });

        if asset_addresses.len() == 0 {
            // call to moralis to get all token data
            let data = ctx.cache.fetch_tokens(address).await;
            result.extend(data);
        } else {
            // For specified token
            let mut assets_without_native: Vec<String> = Vec::new();

            for asset_address in &asset_addresses {
                // Check if all asset addresses are valid Ethereum addresses
                if !is_valid_ethereum_address(asset_address) {
                    return Err(InvalidAddress.into());
                }

                // delete address 0 in token array
                if !is_zero_ethereum_address(asset_address) {
                    assets_without_native.push(asset_address.to_string());
                }
            }

            let balances = get_balances(&address, &assets_without_native).await;
            let decimals = get_decimals(&assets_without_native).await;

            for (i, asset_without_native) in assets_without_native.iter().enumerate() {
                result.push(Balance {
                    asset_address: asset_without_native.to_string(),
                    value: balances[i].clone(),
                    decimals: decimals[i] as i32,
                });
            }
        }

        Ok(result)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
