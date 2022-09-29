use std::env;

const APCA_API_KEY_ID: &'static str = "APCA_API_KEY_ID";
const APCA_API_SECRET_KEY: &'static str = "APCA_API_SECRET_KEY";

pub struct Credentials {
    pub key: String,
    pub secret: String,
}

pub fn get_credentials() -> Result<Credentials, &'static str> {
    let key = env::var(APCA_API_KEY_ID).expect("$APCA_API_KEY_ID is not set");
    let secret = env::var(APCA_API_SECRET_KEY).expect("$APCA_API_SECRET_KEY is not set");

    Ok(Credentials { key, secret })
}
