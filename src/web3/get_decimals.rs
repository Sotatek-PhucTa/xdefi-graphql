use std::str::FromStr;
use web3::api::Eth;
use web3::Web3;
use web3::transports::Http;
use web3::Transport;

use web3::contract::{ Contract, Options };
use web3::ethabi::Token;
use web3::types::{ Address };

use crate::validates::env::{ get_rpc, get_mullticall_address };
use crate::web3::models::decimals::{ MulticallDecimalsResponse };
use crate::web3::utils::{ CallData, parse_addresses };

static MULTICALL_ABI: &[u8] = include_bytes!("../../abis/multicall.json");
static ERC20_ABI: &[u8] = include_bytes!("../../abis/erc20.json").as_slice();

pub async fn get_decimals(tokens_addresses: &Vec<String>) -> Vec<u8> {
    let http = Http::new(&get_rpc().unwrap()).unwrap();
    let web3 = Web3::new(http);

    let parsed_token = parse_addresses(tokens_addresses);
    let tokens_decimals: Vec<u8> = multicall_decimals(web3.eth(), &parsed_token).await;

    for (idx, decimals) in tokens_decimals.iter().enumerate() {
        println!("Token {} of chain has decimals {:?}", &tokens_addresses[idx], decimals);
    }
    tokens_decimals
    // println!("Finished to get balances {}", self.id);
}

async fn multicall_decimals<T: Transport>(eth: Eth<T>, tokens_addresses: &Vec<Address>) -> Vec<u8> {
    let contract_address = Address::from_str(&get_mullticall_address().unwrap()).unwrap();
    let contract_abi = MULTICALL_ABI;

    let token_contract = Contract::from_json(eth, contract_address, contract_abi).unwrap();

    let call_data = get_multicall_decimals_calldata(tokens_addresses);
    let tokens_decimals: MulticallDecimalsResponse = token_contract
        .query("aggregate", call_data, None, Options::default(), None).await
        .unwrap_or_else(|_op| { MulticallDecimalsResponse::default() });

    tokens_decimals.return_data
}

fn get_multicall_decimals_calldata(tokens_addresses: &Vec<Address>) -> Vec<Token> {
    let erc20_contract = web3::ethabi::Contract::load(ERC20_ABI).unwrap();
    let function = erc20_contract.function("decimals").unwrap();
    let erc20_call_data = function.encode_input(&[]).unwrap();

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
