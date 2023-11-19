#[cfg(test)]
mod tests {
    use super::super::serializer::*;

    #[test]
    fn test_str() {
        // Test with a regular string
        assert_eq!(str("Hello"), b"+Hello\r\n");

        // Test with an empty string
        assert_eq!(str(""), b"+\r\n");

        // Test with a string containing special characters
        assert_eq!(str("Special\r\nChars"), b"+Special\r\nChars\r\n");

        // Test with a string containing escaped characters
        assert_eq!(str("\\r\\n"), b"+\\r\\n\r\n");
    }

    #[test]
    fn test_bulk_str() {
        // Test with a regular string
        assert_eq!(bulk_str("Hello"), b"$5\r\nHello\r\n");

        // Test with an empty string
        assert_eq!(bulk_str(""), b"$0\r\n\r\n");

        // Test with a string containing special characters
        assert_eq!(bulk_str("Special\r\nChars"), b"$14\r\nSpecial\r\nChars\r\n");

        // Test with a string containing escaped characters
        assert_eq!(bulk_str("\\r\\n"), b"$4\r\n\\r\\n\r\n");
    }

    #[test]
    fn test_str_arr() {
        // Test with an empty array
        assert_eq!(str_arr(&vec![]), b"*0\r\n");

        // Test with an array of strings
        assert_eq!(
            str_arr(&vec![
                "One".to_string(),
                "Two".to_string(),
                "Three".to_string()
            ]),
            b"*3\r\n$3\r\nOne\r\n$3\r\nTwo\r\n$5\r\nThree\r\n"
        );

        // Test with an array of empty strings
        assert_eq!(
            str_arr(&vec!["".to_string(), "".to_string(), "".to_string()]),
            b"*3\r\n$0\r\n\r\n$0\r\n\r\n$0\r\n\r\n"
        );
    }

    #[test]
    fn test_int() {
        // Test with a positive integer
        assert_eq!(int(42), b":42\r\n");

        // Test with a negative integer
        assert_eq!(int(-123), b":-123\r\n");

        // Test with zero
        assert_eq!(int(0), b":0\r\n");

        // Test with the maximum positive integer
        assert_eq!(int(i64::MAX), format!(":{}\r\n", i64::MAX).as_bytes());
    }

    #[test]
    fn test_err() {
        // Test with a regular error message
        assert_eq!(err("Error Message"), b"-Error Message\r\n");

        // Test with an empty error message
        assert_eq!(err(""), b"-\r\n");

        // Test with a message containing special characters
        assert_eq!(err("Special\r\nChars"), b"-Special\r\nChars\r\n");

        // Test with a message containing escaped characters
        assert_eq!(err("\\r\\n"), b"-\\r\\n\r\n");
    }

    #[test]
    fn test_nil() {
        // Test with nil
        assert_eq!(nil(), b"$-1\r\n");
    }
}
