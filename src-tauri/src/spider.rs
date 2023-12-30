use std::error::Error;
use reqwest::Client;
use scraper::{Selector, Html}; //TODO: use html5ever
use serde_json::Value;
use crate::types::Weather;

const WEATHER_RUL: &str = "http://apis.juhe.cn/simpleWeather/query";
const KEY: &str = "c7345ec9a71317ff4560960f3718c2a0";

#[tauri::command]
pub async fn get_weather() -> Weather { //TODO:运行时怎么处理？TAURI好像自带了TOKIO？
    let client = reqwest::Client::new();
    match spider_weather(&client).await {
        Ok(weather) => weather,
        Err(_) => Weather {
            date: "暂无信息".to_string(),
            direct: "暂无信息".to_string(),
            temperature: "暂无信息".to_string(),
            weather: "暂无信息".to_string(),
        }
    }
}

async fn spider_weather(client: &Client) -> Result<Weather, Box<dyn Error>> {
    let city = "安庆";
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

