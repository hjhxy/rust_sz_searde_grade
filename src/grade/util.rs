use std::io;
use regex::Regex;
use unicode_width::UnicodeWidthStr;

pub fn validate_cookie(cookie: &str) -> Result<(), String>{
    if cookie.is_empty(){
        return Err("cookie 为空".to_string());
    }
    // let re = Regex::new(r"^([a-zA-Z0-9!#$%&'*+.^_`|~-]+=[a-zA-Z0-9!#$%&'*+.^_`|~-]+)(;[a-zA-Z0-9!#$%&'*+.^_`|~-]+=[a-zA-Z0-9!#$%&'*+.^_`|~-]+)*$").unwrap();
    // if re.is_match(cookie) == false {
    //     return Err("cookie 无效".to_string());
    // }
    Ok(())
}


pub fn pad_str(s: &str, width: usize) -> String {
    let real_width = UnicodeWidthStr::width(s);  // 获取真实占用宽度
    if real_width >= width {
        s.to_string()
    } else {
        let padding = width - real_width;
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;
        format!("{}{}{}", " ".repeat(left_padding), s, " ".repeat(right_padding))
    }
}


pub fn before_exit(){
    println!("成绩查询结束，按任意键退出~");
    io::stdin().read_line(&mut String::new()).unwrap();
}