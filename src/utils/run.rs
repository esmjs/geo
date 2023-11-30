use super::{
    get_province, is_json_content_same, print_err, print_link, read_previous_json_file,
    send_http_request, write_to_json_file,
};

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let province: &str = "https://xiaoxian521.github.io/hyperlink/json/province-info.json";
    let pretty_print: bool = true; // 设置是否进行美化输出
    let max_retries: u32 = 3; // 设置最大重试次数

    match get_province(province).await {
        Ok(_data) => {
            for _obj in _data {
                let base_url = "https://geo.datav.aliyun.com/areas_v3/bound/geojson?code=";
                // taiwan的请求参数?code=710000无_full，别的省都带_full
                let full = if _obj.adcode == 710000 { "" } else { "_full" };
                let url: String = format!("{}{}{}", base_url, _obj.adcode, full);
                println!("{:?}", url);
                let filename: String = format!("json/{}.json", _obj.filename);

                let body = send_http_request(url.as_str(), max_retries).await; // 发送HTTP请求并获取响应内容
                let previous_content: Option<String> = read_previous_json_file(&filename)?;

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
            }
        }
        Err(err) => {
            eprintln!("{}\n{}", print_link(province), print_err(err.as_str()))
        }
    }

    Ok(())
}
