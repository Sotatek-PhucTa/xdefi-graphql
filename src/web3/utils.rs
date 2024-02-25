use std::str::FromStr;
use web3::contract::tokens::{ Tokenizable };
use web3::contract::Error;
use web3::ethabi::{ Bytes, Token };
use web3::types::{ Address, H160 };

impl Tokenizable for CallData {
    fn from_token(token: Token) -> Result<Self, Error> where Self: Sized {
        match token {
            Token::Tuple(x) => {
                if x.len() != 2 {
                    return Err(
                        Error::Api(web3::error::Error::Decoder("Invalid tuple".to_string()))
                    );
                }
                Ok(CallData {
                    target: Address::from_token(x[0].clone()).unwrap(),
                    call_data: Bytes::from_token(x[1].clone()).unwrap(),
                })
            }
            _ => Err(Error::Api(web3::error::Error::Decoder("Error".to_string()))),
        }
    }
    fn into_token(self) -> Token {
        Token::Tuple(vec![self.target.into_token(), self.call_data.into_token()])
    }
}
impl CallData {
    pub fn get_param_calldata(self) -> Token {
        self.into_token()
    }
}

pub struct CallData {
    pub target: Address,
    pub call_data: Bytes,
}

pub fn parse_addresses(tokens_addresses: &Vec<String>) -> Vec<H160> {
    tokens_addresses
        .iter()
        .filter_map(|tokens_addresses| {
            Address::from_str(tokens_addresses.as_str())
                .map_err(|op| {
                    println!("Invalid address: {op}");
                    op
                })
                .ok()
        })
        .collect()
}
