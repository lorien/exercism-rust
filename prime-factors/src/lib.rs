use std::cmp;

pub fn get_next_prime(base: u64) -> u64 {
    if base == 0 || base == 1 {
        return 2;
    }
    for num in (base + 1).. {
        let mut ok = true;
        for factor in 2..=cmp::max(2, (num as f64).sqrt() as u64) {
            if num % factor == 0 {
                ok = false;
                break;
            } else {
                // pass
            }
        }
        if ok {
            return num;
        }
    }
    panic!("That should not happen")
}

pub fn factors(inp: u64) -> Vec<u64> {
    //println!("inp: {}", inp);
    let mut ret = vec![];
    let mut num = inp;
    if num != 0 && num != 1 {
        let mut factor = get_next_prime(1);
        while factor <= num {
            if num % factor == 0 {
                ret.push(factor);
                num = num / factor;
            } else {
                factor = get_next_prime(factor);
            }
        }
    }
    //println!("ret: {:?}", ret);
    ret
}
