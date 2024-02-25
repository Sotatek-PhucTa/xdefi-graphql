use regex::Regex;

pub fn is_valid_ethereum_address(addr: &str) -> bool {
    // Ethereum addresses are hexadecimal strings of length 40 (excluding the '0x' prefix)
    if addr.len() != 42 || !addr.starts_with("0x") {
        return false;
    }

    // Check that the characters after '0x' are all hexadecimal
    if !Regex::new("^0x[a-fA-F0-9]+$").unwrap().is_match(addr) {
        return false;
    }

    true
}

// Function to check if an Ethereum address is zero
pub fn is_zero_ethereum_address(address: &str) -> bool {
    // Check if the address consists of all zeros
    address.trim_start_matches("0x").trim_matches('0').is_empty()
}
