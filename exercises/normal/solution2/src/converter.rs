pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 提取出数字部分和原进制部分
    let mut parts = num_str.split('(');
    let num_part = parts.next().unwrap();
    let from_base_str = parts.next().unwrap().trim_end_matches(')');
    let from_base: u32 = from_base_str.parse().unwrap();
    // 数字部分原进制转换为十进制
    let decimal_num: u32 = u32::from_str_radix(num_part, from_base).unwrap();
    // 十进制数字转换为目标进制
    if to_base == 10 {
        return decimal_num.to_string();
    } else {
        let mut result = String::new();
        let mut temp_num = decimal_num;
        while temp_num > 0 {
            let remainder = temp_num % to_base;
            if remainder < 10 {
                result.push(char::from(remainder as u8 + b'0'));
            } else {
                result.push(char::from(remainder as u8 + b'a' - 10));
            }
            temp_num /= to_base;
        }
        result.chars().rev().collect::<String>()
    }
}