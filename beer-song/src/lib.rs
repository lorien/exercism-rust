pub fn verse(num: u32) -> String {
    if num == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else {
        let mut tail = String::new();
        let mut tail_suffix = String::new();
        let mut plural_suffix = String::new();
        if num - 1 > 1 {
            tail_suffix = "s".to_string();
        }
        if num > 1 {
            tail = format!(
                "Take one down and pass it around, {} bottle{} of beer on the wall.",
                num - 1, tail_suffix
            );
            plural_suffix = "s".to_string();
        } else {
            tail = "Take it down and pass it around, no more bottles of beer on the wall.".to_string()
        }
        format!(
            "{} bottle{} of beer on the wall, {} bottle{} of beer.\n\
            {}\n",
            num, plural_suffix, num, plural_suffix, tail
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret = String::new();
    for num in (end..=start).rev() {
        ret.push_str(&verse(num));
        if num != end {
            ret.push_str("\n")
        }
    }
    ret
}
