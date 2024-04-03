fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrt = (n as f64).sqrt() as i32;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}
