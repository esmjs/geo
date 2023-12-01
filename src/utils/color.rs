use nu_ansi_term::{
    AnsiGenericString,
    Color::{Blue, Green, LightYellow, Red, White, Yellow},
};

pub fn print_link(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("无效URL：{}", text);
    return Blue.paint(_text);
}

pub fn print_err(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("错误信息：{}", text);
    return Red.paint(_text);
}

pub fn print_fail(text: u32, max: u32) -> AnsiGenericString<'static, str> {
    let _text: String = format!("请求失败，正在重试... (第 {}/{})", text, max);
    return Yellow.paint(_text);
}

pub fn print_out(url: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!(
        "超过最大重试次数，请检查请求失败的URL是否有误：{} \n程序已退出",
        Blue.underline().paint(url)
    );
    return LightYellow.paint(_text);
}

pub fn print_filename(filename: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("{} 无变化，无需更新", filename);
    return White.paint(_text);
}

pub fn print_write_filename(filename: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("{} 写入成功", filename);
    return Green.paint(_text);
}

pub fn print_update_filename(filename: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("{} 有变化，已经更新", filename);
    return Green.paint(_text);
}
