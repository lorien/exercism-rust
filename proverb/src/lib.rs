pub fn build_proverb(list: &[&str]) -> String {
    let mut ret = String::new();
    if list.len() > 0 {
        for idx in 0..(list.len() - 1) {
            ret.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[idx],
                list[idx + 1]
            ))
        }
        ret.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    ret
}
