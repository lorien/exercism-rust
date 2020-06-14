pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers = Vec::new();
    for factor in factors {
        if *factor == 0 {
            // pass
        } else {
            for mult in 1..=((limit / factor) as u32) {
                let mut ret = factor * mult;
                if ret < limit {
                    if ! numbers.contains(&ret) {
                        numbers.push(ret);
                    }
                } else {
                    break
                }
            }
        }
    }
    numbers.iter().sum()
}
