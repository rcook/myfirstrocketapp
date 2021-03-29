use crate::result::Result;

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn run_experiments() -> Result<()> {
    // Grab a token from the auth server
    let token = auth_server::test_encode()?;

    println!("token={:?}", token);

    // Validate it on the resource server
    resource_server::test_decode_and_validate(&token)?;

    Ok(())
}

// Demonstrate how an auth server generates a JWT
mod auth_server {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};
    use std::convert::TryFrom;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use super::Token;
    use crate::result::Result;

    fn exp_from_duration(now: SystemTime, duration: Duration) -> Result<usize> {
        let expiry_time = now.checked_add(duration)?;
        Ok(usize::try_from(
            expiry_time.duration_since(UNIX_EPOCH)?.as_secs(),
        )?)
    }

    pub fn test_encode() -> Result<Token> {
        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            company: String,
            exp: usize,
        }

        let claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: exp_from_duration(SystemTime::now(), Duration::from_secs(3600))?,
        };

        let token = Token::new(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )?);

        Ok(token)
    }
}

// Demonstrate how a resource server validates a JWT
mod resource_server {
    use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
    use serde::{Deserialize, Serialize};

    use super::Token;
    use crate::result::Result;

    pub fn test_decode_and_validate(token: &Token) -> Result<()> {
        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            company: String,
            exp: usize,
        }

        // Claims is a struct that implements Deserialize
        let token_data = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;

        println!("token_data={:?}", token_data);

        Ok(())
    }
}
