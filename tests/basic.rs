#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        let result = rmonkey::execute("tests/example/basic.monkey");
        assert_eq!(result, "5");
    }
}
