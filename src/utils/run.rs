use path_abs::{PathAbs, PathInfo};
use std::process;

use super::{
    create_folder_if_not_exists, get_province, is_json_content_same, print_err, print_filename,
    print_link, print_out, print_write_filename, read_previous_json_file, send_http_request,
    write_to_json_file,
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

                let abs_path = PathAbs::new("json")?;
                create_folder_if_not_exists(abs_path.clone())?;
                let filename: String = format!("{}/{}.json", abs_path.display(), _obj.filename);
                let relative_filename: String = format!("json/{}.json", _obj.filename);

                let body = send_http_request(url.as_str(), max_retries).await;
                let previous_content: Option<String> = read_previous_json_file(&filename)?;

                match body {
                    Ok(body) => {
                        if let Some(previous) = previous_content {
                            if is_json_content_same(&previous, &body) {
                                println!("{}", print_filename(&relative_filename));
                            } else {
                                write_to_json_file(&filename, &body, pretty_print)?;
                                println!("{}", print_write_filename(&relative_filename));
                            }
                        } else {
                            write_to_json_file(&filename, &body, pretty_print)?;
                            println!("{}", print_write_filename(&relative_filename));
                        }
                    }
                    Err(err) => {
                        eprintln!("{}", print_out(&err));
                        process::exit(1);
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
