mod decode_token;
pub use decode_token::decode_token;
mod test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_jwt() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHBsaWNhdGlvbiI6ImJvbmdrb3ctd2FsbGV0cyIsImFkZHJlc3MiOiIweDgzYTZjZjlmMzdjM2UzYzRjZTU2NDkwNWM3ODM2MmIwZjc3NzcwNzIiLCJpYXQiOjE3NDU0NTg5MTAsImV4cCI6MTc0NTU0NTMxMCwiaXNzIjoiYm9uZ2tvdy13YWxsZXRzIn0.MZ5jaSQ804XaAL1LzmlRsBhZ5CLjPAHGdoZUUioK9IQ";
        let secret_key = "011c7edd154662b474f9f3bec4079be849fc134edcf4322282219046330d25eb";
        let result = decode_token(token, secret_key);
        println!("{:?}", result);
    }
}
