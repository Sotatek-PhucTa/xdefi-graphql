use web3::contract::tokens::{ Detokenize };
use web3::contract::Error;
use web3::ethabi::{ Token };
use web3::types::{ U256 };

#[derive(Default)]
pub struct MulticallBalanceResponse {
    pub return_data: Vec<String>,
}

impl Detokenize for MulticallBalanceResponse {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error> where Self: Sized {
        /* Response:
        [block_number, [Response, Response, Response]]
        */
        let return_data = tokens[1].clone().into_array().unwrap();

        let token_balances: Vec<String> = return_data
            .iter()
            .map(|response| {
                match response {
                    Token::Bytes(x) => {
                        if x.len() == 0 {
                            "Invalid token address".into()
                        } else {
                            U256::from_big_endian(&x).to_string()
                        }
                    }
                    _ => "Invalid response type".into(),
                }
            })
            .collect();

        Ok(Self {
            return_data: token_balances,
        })
    }
}
