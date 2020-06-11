/* Problem 3:
    Largest prime factor

    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
*/

pub fn solve(value: u64) -> u64 {
    let prime_factors = calculate_prime_factors(value);
    
    *prime_factors.last().unwrap()
}

fn calculate_prime_factors(number: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut upper_limit = number / 2;
    let mut divisor = 2;

    while divisor < upper_limit {
        if number % divisor == 0 && is_prime(&divisor) {
            prime_factors.push(divisor);
        }

        divisor += 1;
        upper_limit = number / divisor;
    }

    prime_factors
}

fn is_prime(number: &u64) -> bool {
    let mut upper_limit = number / 2;
    let mut divisor = 2;

    while divisor < upper_limit {
        if number % divisor == 0 {
            return false
        }

        divisor += 1;
        upper_limit = number / divisor;
    }

    true
}

#[cfg(test)]
mod p3_tests {
    use super::*;

    #[test]
    fn knows_when_prime() {
        assert!( is_prime(&5) );
        assert!( is_prime(&7) );
        assert!( is_prime(&13) );
        assert!( is_prime(&677) );
    }

    #[test]
    fn knows_when_not_prime() {
        assert!( !is_prime(&6) );
        assert!( !is_prime(&8) );
        assert!( !is_prime(&14) );
        assert!( !is_prime(&678) );
    }

    #[test]
    fn calculates_correct_prime_factors() {
        assert_eq!(
            vec![5, 7, 13, 29],
            calculate_prime_factors(13195),
        );
    }

    #[test]
    fn euler_example() {
        assert_eq!(
            29,
            solve(13195),
        );
    }
}