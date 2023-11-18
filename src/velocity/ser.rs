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

pub fn nil() -> Vec<u8> {
    let value = resp::Value::Null;
    let encoded = resp::encode(&value);
    let response: String = String::from_utf8_lossy(&encoded).into_owned();
    let response = response.as_bytes();

    response.to_owned()
}
