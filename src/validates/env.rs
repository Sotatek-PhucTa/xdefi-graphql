use std::env;
use dotenv::dotenv;
use crate::errors::Error;

pub fn get_rpc() -> Result<String, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let get_rpc;
    match env::var("RPC") {
        Ok(value) => {
            get_rpc = value;
        }
        Err(_) => {
            return Err(Error::InvalidEnv.into());
        }
    }
    println!("{}", get_rpc);
    Ok(get_rpc)
}

pub fn get_mullticall_address() -> Result<String, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let get_multicall;
    match env::var("MULTICALL_ADDRESS") {
        Ok(value) => {
            get_multicall = value;
        }
        Err(_) => {
            return Err(Error::InvalidEnv.into());
        }
    }
    println!("{}", get_multicall);
    Ok(get_multicall)
}
