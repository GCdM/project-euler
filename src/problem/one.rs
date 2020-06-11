/* Problem 1: 
    Multiples of 3 and 5

    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

    Find the sum of all the multiples of 3 or 5 below 1000. 
*/

pub fn solve(upper_limit: i32) -> i32 {
    let mut sum = 0;
    
    for number in 1..upper_limit {
        if is_divisible_by_3(&number) || is_divisible_by_5(&number) {
            sum += number;
        }
    }
    
    sum
}

fn is_divisible_by_3(value: &i32) -> bool {
    value % 3 == 0
}

fn is_divisible_by_5(value: &i32) -> bool {
    value % 5 == 0
}

#[cfg(test)]
mod p1_tests {
    use super::*;

    #[test]
    fn divides_by_3() {
        assert!( is_divisible_by_3(&3) );
        assert!( is_divisible_by_3(&6) );
        assert!( is_divisible_by_3(&9) );
        assert!( is_divisible_by_3(&27) );
    }

    #[test]
    fn divides_by_5() {
        assert!( is_divisible_by_5(&5) );
        assert!( is_divisible_by_5(&10) );
        assert!( is_divisible_by_5(&25) );
        assert!( is_divisible_by_5(&125) );
    }

    #[test]
    fn knows_indivisible() {
        assert!( !is_divisible_by_3(&1) );
        assert!( !is_divisible_by_3(&4) );
        assert!( !is_divisible_by_3(&238) );
        assert!( !is_divisible_by_5(&1) );
        assert!( !is_divisible_by_5(&4) );
        assert!( !is_divisible_by_5(&238) );
    }

    #[test]
    fn euler_example() {
        assert_eq!( 23, solve(10) )
    }
    
}