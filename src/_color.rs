use nu_ansi_term::{
    AnsiGenericString,
    Color::{Blue, Red},
    // Color::{Blue, Cyan, Red, White},
};

pub fn print_link(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("无效URL：{}", text);
    println!(); // 打印一个空行
    return Blue.paint(_text);
    // return Cyan.on(Blue).fg(White).paint(_text);
}

pub fn print_err(text: &str) -> AnsiGenericString<'_, str> {
    let _text: String = format!("错误信息：{}", text);
    println!();
    return Red.paint(_text);
    // return Cyan.on(Red).fg(White).paint(_text);
}
