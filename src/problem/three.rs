/* Problem 3:
    Largest prime factor

    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
*/

pub fn solve(value: u64) -> u64 {

    0
}

fn calculate_factors(value: u64) -> Vec<u64> {

    vec![]
}

fn is_prime(number: &u64) -> bool {

    false
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
    fn gets_correct_factors() {
        assert_eq!(
            vec![5, 7, 13, 29, 35, 65, 91, 145, 203, 377, 455, 1015, 1885, 2639],
            calculate_factors(13195),
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