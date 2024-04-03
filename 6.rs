fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut prefix = strs[0].to_string();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            prefix.pop();
        }
    }
    prefix
}
