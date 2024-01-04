use std::error::Error;
use reqwest::Client;
use serde_json::Value;
use crate::types::Weather;

const WEATHER_RUL: &str = "http://apis.juhe.cn/simpleWeather/query";
const KEY: &str = "c7345ec9a71317ff4560960f3718c2a0";

#[tauri::command]
pub async fn get_weather(city: String) -> Weather { 
    let client = reqwest::Client::new();
    match spider_weather(&client, city).await {
        Ok(weather) => weather,
        Err(_) => Weather::new()
    }
}

async fn spider_weather(client: &Client, city: String) -> Result<Weather, Box<dyn Error>> {
    let url = format!("{}?key={}&city={}", WEATHER_RUL, KEY, city);
    let resp = client
        .get(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?
        .text()
        .await?;
    let result: Value = serde_json::from_str::<Value>(&resp)?["result"]["future"][0].to_owned();
    let weather: Weather = serde_json::from_value(result)?;
    Ok(weather)
}

