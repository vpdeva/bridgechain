
pub mod chains;
pub mod utils;

pub fn transfer_token(chain_from: &str, chain_to: &str, amount: f64) -> Result<String, String> {
    if chain_from.is_empty() || chain_to.is_empty() {
        return Err("Chain names cannot be empty".to_string());
    }
    Ok(format!("Transferred {} tokens from {} to {}", amount, chain_from, chain_to))
}
        