const SMALL_PRIMES: &'static [u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997
];

const MAX_SMALL_PRIME: u64 = SMALL_PRIMES[SMALL_PRIMES.len() - 1];

pub mod prime {
    use crate::{SMALL_PRIMES, MAX_SMALL_PRIME};
    use std::collections::HashMap;

    pub fn is_prime(n: u64) -> bool {
        if n <= MAX_SMALL_PRIME {
            return SMALL_PRIMES.contains(&n);
        } else {
            let sqrt = (n as f64).sqrt() as u64;

            (3..=sqrt).step_by(2)
                .all(|divisor| n % divisor != 0)
        }
    }

    pub fn factors(n: u64) -> HashMap<u64, u64> {
        let mut out = HashMap::new();

        if n == 1 {
            out.insert(2,0);
            return out;
        }

        let mut accumulator = n;
        let mut counter = 2;

        while !is_prime(accumulator) {
            for i in counter..std::u64::MAX {
                counter = i;

                if is_prime(i) && accumulator % i == 0 {
                    accumulator /= i;
                    *out.entry(i).or_insert(0) += 1;
                    break;
                }
            }
        }

        *out.entry(accumulator).or_insert(0) += 1;

        out
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn factors_of_12() {
            let factors = super::factors(12);
            assert_eq!(2, factors.len());
            assert_eq!(2, *factors.get(&2).unwrap());
            assert_eq!(1, *factors.get(&3).unwrap());
        }

        #[test]
        fn factors_of_28() {
            let factors = super::factors(28);
            assert_eq!(2, factors.len());
            assert_eq!(2, *factors.get(&2).unwrap());
            assert_eq!(1, *factors.get(&7).unwrap());
        }
    }
}