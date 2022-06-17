// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {
    let mut num = 0;

    'main: loop {
        num += 1;

        for i in 1..=20 {
            if num % i != 0 {
                continue 'main;
            }
        }

        break;
    };

    println!("{}", num);
}