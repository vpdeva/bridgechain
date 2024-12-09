
pub fn validate_amount(amount: f64) -> Result<(), String> {
    if amount <= 0.0 {
        Err("Amount must be greater than zero".to_string())
    } else {
        Ok(())
    }
}
        