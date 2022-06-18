// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?

fn main() {
    let mut prime = 2;

    let mut x = 0;

    'main: while x < 10000 {
        prime += 1;

        for i in 2..prime {
            if prime % i == 0 {
                continue 'main;
            }
        }

        x += 1;
    }

    println!("{}", prime);
}