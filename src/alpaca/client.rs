use crate::options::credentials::get_credentials;
use reqwest::{header, Client};

//
pub fn get_client() -> Result<Client, &'static str> {
    let headers = make_credential_headers();
    let client = Client::builder().default_headers(headers).build().unwrap();
    return Ok(client);
}

fn make_credential_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    let credentials = get_credentials().unwrap();

    headers.insert(
        "APCA-API-KEY-ID",
        header::HeaderValue::from_str(&credentials.key).unwrap(),
    );

    headers.insert(
        "APCA-API-SECRET-KEY",
        header::HeaderValue::from_str(&credentials.secret).unwrap(),
    );

    return headers;
}
