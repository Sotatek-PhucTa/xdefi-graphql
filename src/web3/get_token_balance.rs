use std::str::FromStr;
use web3::api::Eth;
use web3::Web3;
use web3::transports::Http;
use web3::Transport;

use web3::contract::{ Contract, Options };
use web3::ethabi::Token;
use web3::types::{ Address };

use crate::validates::env::{ get_rpc, get_mullticall_address };
use crate::web3::models::balance::{ MulticallBalanceResponse };
use crate::web3::utils::{ CallData, parse_addresses };

static MULTICALL_ABI: &[u8] = include_bytes!("../../abis/multicall.json");
static ERC20_ABI: &[u8] = include_bytes!("../../abis/erc20.json").as_slice();

pub async fn get_balances(address: &str, tokens_addresses: &Vec<String>) -> Vec<String> {
    let http = Http::new(&get_rpc().unwrap()).unwrap();
    let web3 = Web3::new(http);

    let caller_address = Address::from_str(address).unwrap();
    let parsed_token = parse_addresses(tokens_addresses);
    let tokens_balance: Vec<String> = multicall_balances(
        web3.eth(),
        caller_address,
        &parsed_token
    ).await;

    for (idx, balance) in tokens_balance.iter().enumerate() {
        println!("Token {} of chain has balanceOf {:?}", &tokens_addresses[idx], balance);
    }
    tokens_balance
    // println!("Finished to get balances {}", self.id);
}

async fn multicall_balances<T: Transport>(
    eth: Eth<T>,
    caller_address: Address,
    tokens_addresses: &Vec<Address>
) -> Vec<String> {
    let contract_address = Address::from_str(&get_mullticall_address().unwrap()).unwrap();
    let contract_abi = MULTICALL_ABI;

    let token_contract = Contract::from_json(eth, contract_address, contract_abi).unwrap();

    let call_data = get_multicall_balance_calldata(caller_address, tokens_addresses);
    let tokens_balance: MulticallBalanceResponse = token_contract
        .query("aggregate", call_data, None, Options::default(), None).await
        .unwrap_or_else(|_op| { MulticallBalanceResponse::default() });

    tokens_balance.return_data
}

fn get_multicall_balance_calldata(
    caller_address: Address,
    tokens_addresses: &Vec<Address>
) -> Vec<Token> {
    let erc20_contract = web3::ethabi::Contract::load(ERC20_ABI).unwrap();
    let function = erc20_contract.function("balanceOf").unwrap();
    let erc20_call_data = function.encode_input(&[Token::Address(caller_address)]).unwrap();

    tokens_addresses
        .iter()
        .map(|x| {
            (CallData {
                target: x.clone(),
                call_data: erc20_call_data.clone(),
            }).get_param_calldata()
        })
        .collect::<Vec<Token>>()
}
