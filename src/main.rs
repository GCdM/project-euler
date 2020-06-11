mod problem;

fn main() {

    /* Problem 1: 
        Multiples of 3 and 5 */ 
    let input_one = 1000;
    println!(
        "Solution 1: ({}) -> {}", 
        input_one, 
        problem::one::solve(input_one),
    );

    /* Problem 2:
        Even Fibonacci numbers */
    let input_two = 4000000;
    println!(
        "Solution 2: ({}) -> {}",
        input_two,
        problem::two::solve(input_two),
    );

    /* Problem 2:
        Even Fibonacci numbers */
    let input_three = 600851475143;
    println!("
        Solution 3: ({}) -> {}",
        input_three,
        problem::three::solve(input_three),
    );
}
