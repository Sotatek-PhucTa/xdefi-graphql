use web3::contract::tokens::{ Detokenize };
use web3::contract::Error;
use web3::ethabi::{ Token };
use web3::types::{ U256 };

#[derive(Default)]
pub struct MulticallDecimalsResponse {
    pub return_data: Vec<u8>,
}

impl Detokenize for MulticallDecimalsResponse {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error> where Self: Sized {
        /* Response:
        [block_number, [Response, Response, Response]]
        */

        let return_data = tokens[1].clone().into_array().unwrap();

        let token_decimals = return_data
            .iter()
            .map(|response| {
                match response {
                    Token::Bytes(x) => U256::from_big_endian(&x).low_u32() as u8,
                    _ => U256([0, 0, 0, 0]).low_u32() as u8,
                }
            })
            .collect();

        Ok(Self {
            return_data: token_decimals,
        })
    }
}
