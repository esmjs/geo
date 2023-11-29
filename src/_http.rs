use reqwest;
use std::time::Duration;

pub async fn send_http_request(
    url: &str,
    max_retries: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10)) // 设置超时时间为10秒
        .build()?;

    let mut retries: u32 = 0;

    loop {
        let response = client.get(url).send().await?; // 发送GET请求

        // 检查请求的状态码
        if response.status().is_success() {
            let body = response.text().await?; // 获取响应的文本内容
            return Ok(body);
        }

        retries += 1;

        if retries > max_retries {
            return Err("超过最大重试次数".into());
        }

        println!("请求失败，正在重试... (第 {}/{})", retries, max_retries);

        tokio::time::sleep(Duration::from_secs(1)).await; // 等待1秒后进行下一次请求
    }
}
