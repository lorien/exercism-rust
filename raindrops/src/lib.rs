pub fn raindrops(num: u32) -> String {
    let mut ret = String::from("");
    if num % 3 == 0 {
        ret.push_str("Pling");
    }
    if num % 5 == 0 {
        ret.push_str("Plang");
    }
    if num % 7 == 0 {
        ret.push_str("Plong");
    }
    if ret.is_empty() {
        ret.push_str(&num.to_string());
    }
    ret
}
