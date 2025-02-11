use unicode_width::UnicodeWidthStr;

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
