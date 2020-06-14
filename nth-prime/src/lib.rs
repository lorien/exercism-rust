pub fn nth(req_idx: u32) -> u32 {
    let mut prime_idx = 0;
    let mut num = 2;
    let mut primes = vec![2];

    loop {
        num += 1;
        let mut is_prime = true;
        for prime in &primes {
            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_idx += 1;
            if prime_idx == req_idx {
                return num
            }
            primes.push(num);
        }
    }
}
