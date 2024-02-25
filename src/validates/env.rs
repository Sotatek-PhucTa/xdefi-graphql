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
    Ok(get_multicall)
}

pub fn get_moralis_key() -> Result<String, Error> {
    // Load environment variables from the .env file
    dotenv().ok();
    let get_moalis;
    match env::var("MORALIS_KEY") {
        Ok(value) => {
            get_moalis = value;
        }
        Err(_) => {
            return Err(Error::InvalidEnv.into());
        }
    }
    Ok(get_moalis)
}
