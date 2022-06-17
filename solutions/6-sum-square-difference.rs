// The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385

// The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025

// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2650.

// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let mut sum_of_squares = 0;
    let mut square_of_sum = 0;

    for i in 0..=100 {
        sum_of_squares += i * i;
        square_of_sum += i;
    }

    square_of_sum *= square_of_sum;

    let difference = square_of_sum - sum_of_squares;

    println!("{}", difference);
}