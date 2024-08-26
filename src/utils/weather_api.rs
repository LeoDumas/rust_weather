use reqwest::Error;
use serde_json::Value;

pub async fn get_lat_long(city: String) -> Result<Value, Error> {
    let uri = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        city
    );
    // Perform the HTTP GET request
    let resp = reqwest::get(&uri).await?;
    // Parse the response body as JSON
    let json: Value = resp.json().await?;
    // Return the JSON value
    Ok(json)
}
