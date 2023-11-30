use nu_ansi_term::{
    AnsiGenericString,
    Color::{Blue, Red, Yellow},
};

pub fn print_link(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("无效URL：{}", text);
    println!(); // 打印一个空行
    return Blue.paint(_text);
}

pub fn print_err(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("错误信息：{}", text);
    println!();
    return Red.paint(_text);
}

pub fn print_fail(text: u32, max: u32) -> AnsiGenericString<'static, str> {
    let _text: String = format!("请求失败，正在重试... (第 {}/{})", text, max);
    return Yellow.paint(_text);
}
