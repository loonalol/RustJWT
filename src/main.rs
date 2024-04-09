#[allow(dead_code)]
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    iat: usize,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct Config {
    apns_key_id: String,
    team_id: String,
}

fn generate_token(config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let twenty_four_hours = Duration::from_secs(60 * 60 * 24);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    let tomorrow = now + twenty_four_hours.as_secs() as usize;

    let claims = Claims {
        iss: config.team_id.clone(),
        iat: now,
        exp: tomorrow,
    };

    let private_key = fs::read_to_string("AuthKey_9647W8QUMQ.p8").expect("Failed to read private key");

    let token = encode(
        &Header::new(Algorithm::ES256),
        &claims,
        &EncodingKey::from_ec_pem(private_key.as_bytes()).unwrap(),
    )?;

    Ok(token)
}

fn main() {
    let config_text = fs::read_to_string("jwt.json").expect("Failed to read config file");
    let config: Config = serde_json::from_str(&config_text).expect("Error parsing config file");

    match generate_token(&config) {
        Ok(token) => println!("Generated token: {}", token),
        Err(e) => println!("Error generating token: {:?}", e),
    }
}
