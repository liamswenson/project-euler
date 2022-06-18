// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

// Find the sum of all the primes below two million.

fn main() {
    let mut sum: u64 = 0;

    'main: for i in 2..=2000000 {
        for j in 2..i {
            if i % j == 0 {
                continue 'main;
            }
        }

        println!("{}", i);
        sum += i;
    }

    println!("{}", sum);
}