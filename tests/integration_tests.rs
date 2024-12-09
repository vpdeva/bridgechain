
#[cfg(test)]
mod tests {
    use crate::{transfer_token, utils::validate_amount};

    #[test]
    fn test_token_transfer() {
        let result = transfer_token("Ethereum", "Polygon", 100.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount(10.0).is_ok());
        assert!(validate_amount(0.0).is_err());
    }
}
        