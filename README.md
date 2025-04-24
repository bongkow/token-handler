# Token Handler

A Rust library for handling JWT tokens with simple decoding functionality.

## Features

- JWT token decoding with validation
- Support for standard JWT claims

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
token-handler = "0.1.0"
```

## Usage

```rust
use token_handler::decode_token;

fn main() {
    let token = "your.jwt.token";
    let secret_key = "your-secret-key";
    
    match decode_token(token, secret_key) {
        Ok(claims) => println!("Token decoded successfully: {:?}", claims),
        Err(err) => println!("Failed to decode token: {}", err),
    }
}
```

## Token Structure

The decoder expects tokens with the following claims:

- `application`: The application identifier
- `address`: User address
- `iat`: Issued at timestamp
- `exp`: Expiration timestamp
- `iss`: Issuer information

## License

MIT
