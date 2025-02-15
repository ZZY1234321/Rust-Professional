use std::collections::HashMap;

pub fn new_count_distinct(input_str: &str) -> usize {
    let words: Vec<&str> = input_str.split(',').collect();
    // 创建一个空的HashMap，键为单词，值为出现次数
    let mut word_count: HashMap<&str, usize> = HashMap::new();
    
    for word in words {
        if let Some(count) = word_count.get_mut(word) {
            *count += 1;
        } else {
            word_count.insert(word, 1);
        }
    }
    word_count.len()
}