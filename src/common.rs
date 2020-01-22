pub mod prime {
    pub fn is_prime(n: u64) -> bool {
        match n {
            1 => true,
            2 => true,
            3 => true,
            5 => true,
            7 => true,
            _ => {
                let sqrt = (n as f64).sqrt() as u64;

                (3..=sqrt).step_by(2)
                    .all(|divisor| n % divisor != 0)
            }
        }
    }
}