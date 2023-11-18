pub fn str(message: &str) -> Vec<u8> {
    let value = resp::Value::String(message.to_string());
    let encoded = resp::encode(&value);
    let response: String = String::from_utf8_lossy(&encoded).into_owned();
    let response = response.as_bytes();

    response.to_owned()
}

pub fn bulk_str(message: &str) -> Vec<u8> {
    let value = resp::Value::BufBulk(message.as_bytes().to_vec());
    let encoded = resp::encode(&value);
    let response: String = String::from_utf8_lossy(&encoded).into_owned();
    let response = response.as_bytes();

    response.to_owned()
}

pub fn int(message: i64) -> Vec<u8> {
    let value = resp::Value::Integer(message);
    let encoded = resp::encode(&value);
    let response: String = String::from_utf8_lossy(&encoded).into_owned();
    let response = response.as_bytes();

    response.to_owned()
}

pub fn err(message: &str) -> Vec<u8> {
    let value = resp::Value::Error(message.to_string());
    let encoded = resp::encode(&value);
    let response: String = String::from_utf8_lossy(&encoded).into_owned();
    let response = response.as_bytes();

    response.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let expected = b"+OK\r\n".to_vec();
        let actual = str("OK");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bulk_str() {
        let expected = b"$3\r\nfoo\r\n".to_vec();
        let actual = bulk_str("foo");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_int() {
        let expected = b":1000\r\n".to_vec();
        let actual = int(1000);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_err() {
        let expected = b"-Error message\r\n".to_vec();
        let actual = err("Error message");

        assert_eq!(expected, actual);
    }
}
