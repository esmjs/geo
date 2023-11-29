mod _fs;
mod _http;

use _fs::{is_json_content_same, read_previous_json_file, write_to_json_file};
use _http::send_http_request;

/**
 * 目前主要功能如下，后续再拓展
 * 发送HTTP请求
 * 调整超时时间
 * 处理失败重试
 * 最大重试次数
 * 美化输出选项
 * JSON差异检测
 * 写入JSON文件
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url: &str = "https://geo.datav.aliyun.com/areas_v3/bound/geojson?code=100000_full";
    let pretty_print: bool = true; // 设置是否进行美化输出
    let max_retries: u32 = 3; // 设置最大重试次数
    let filename: &str = "./build/geo-china.json"; // JSON 文件相对路径

    let body: Result<std::string::String, Box<dyn std::error::Error>> =
        send_http_request(url, max_retries).await; // 发送HTTP请求并获取响应内容
    let previous_content = read_previous_json_file(&filename)?;

    match body {
        Ok(body) => {
            if let Some(previous) = previous_content {
                if is_json_content_same(&previous, &body) {
                    println!("内容与前一个文件相同，无需写入");
                } else {
                    write_to_json_file(&filename, &body, pretty_print)?;
                    println!("响应内容已写入 {} 文件", &filename);
                }
            } else {
                write_to_json_file(&filename, &body, pretty_print)?;
                println!("响应内容已写入 {} 文件", &filename);
            }
        }
        Err(err) => {
            eprintln!("发送HTTP请求时发生错误: {}", err);
        }
    }

    Ok(())
}
