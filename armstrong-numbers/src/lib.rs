pub fn is_armstrong_number(num: u32) -> bool {
    let mut ptr = num;
    let mut digits = vec![];
    loop {
        if ptr >= 10 {
            let rem = ptr % 10;
            ptr = ptr / 10;
            digits.push(rem);
        } else {
            digits.push(ptr);
            break
        }
    }
    let num_digits = digits.len();
    num == digits.iter().map(|x| x.pow(num_digits as u32)).sum()
}
