#[cfg(test)]
mod tests {
    #[test]
    fn test_integer() {
        let result = rmonkey::execute("tests/codes/integer.monkey");
        assert_eq!(result, "2");
    }

    #[test]
    fn test_boolean() {
        let result = rmonkey::execute("tests/codes/boolean.monkey");
        assert_eq!(result, "true");
    }

    #[test]
    fn test_string() {
        let result = rmonkey::execute("tests/codes/string.monkey");
        assert_eq!(result, r#""The Monkey programming languages""#);
    }

    #[test]
    fn test_array() {
        let result = rmonkey::execute("tests/codes/array.monkey");
        assert_eq!(result, r#""Anna""#);
    }

    // #[test]
    // fn test_hash() {
    //     let result = rmonkey::execute("tests/codes/hash.monkey");
    //     assert_eq!(result, r#""Anna""#);
    // }
}
