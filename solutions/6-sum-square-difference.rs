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