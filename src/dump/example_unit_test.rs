#[allow(dead_code)]
pub fn is_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_true() {
        assert_eq!(is_true(), true)
    }

    #[test]
    fn test_is_not_true() {
        assert_eq!(!is_true(), false)
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(is_true(), false)
    }
}
