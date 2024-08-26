use serde::Deserialize;
use std::io;

mod utils;

#[derive(Debug, Deserialize)]
struct HourlyUnits {
    temperature_2m: String,
}

#[derive(Debug, Deserialize)]
struct HourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    timezone: String,
    hourly_units: HourlyUnits,
    hourly: HourlyData,
}

#[derive(Debug, Deserialize)]
struct JsonResult {
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize)]
struct Response {
    results: Vec<JsonResult>,
}

#[tokio::main]
async fn main() {
    println!("Enter your city: ");
    let mut user_city = String::new();

    io::stdin()
        .read_line(&mut user_city)
        .expect("Failed to read user input");

    let user_city = user_city.trim();

    match utils::weather_api::get_lat_long(user_city.to_string()).await {
        Ok(json_value) => {
            let parsed: Response =
                serde_json::from_value(json_value).expect("Failed to parse JSON");

            for result in parsed.results {
                let weather_api_uri = format!(
                    "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
                    result.latitude, result.longitude
                );

                let weather_response = reqwest::get(&weather_api_uri)
                    .await
                    .expect("Failed to fetch weather data")
                    .json::<WeatherResponse>()
                    .await
                    .expect("Failed to parse weather data");

                println!(
                    "Weather data for {} ({}):",
                    result.country, weather_response.timezone
                );

                // Display the hourly weather data
                for (time, temp) in weather_response
                    .hourly
                    .time
                    .iter()
                    .zip(weather_response.hourly.temperature_2m.iter())
                {
                    println!(
                        "Time: {}, Temperature: {}{}",
                        time, temp, weather_response.hourly_units.temperature_2m
                    );
                }

                println!("--------------------------------");
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
