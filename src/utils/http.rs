use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::print_fail;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvinceData {
    pub adcode: i32,
    pub filename: String,
}

pub async fn send_http_request(
    url: &str,
    max_retries: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client: reqwest::Client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10)) // 设置超时时间为10秒
        .build()?;

    let mut retries: u32 = 0;

    loop {
        let response: reqwest::Response = client.get(url).send().await?; // 发送GET请求

        // 检查请求的状态码
        if response.status().is_success() {
            let body: String = response.text().await?; // 获取响应的文本内容
            return Ok(body);
        }

        retries += 1;

        if retries > max_retries {
            return Err("超过最大重试次数".into());
        }

        println!("{}", print_fail(retries, max_retries));

        tokio::time::sleep(Duration::from_secs(1)).await; // 等待1秒后进行下一次请求
    }
}

pub async fn get_province(url: &str) -> Result<Vec<ProvinceData>, String> {
    let response: reqwest::Response = reqwest::get(url).await.unwrap();
    let data: Vec<ProvinceData> = response
        .json()
        .await
        .map_err(|err: reqwest::Error| err.to_string())?;
    Ok(data)
}