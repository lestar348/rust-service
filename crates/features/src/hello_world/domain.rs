/// Builds the hello world response message.
pub fn get_message(now_rfc3339: &str) -> String {
    format!("{now_rfc3339} hello world")
}
