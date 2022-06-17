fn main() {
    let mut prime = 0;

    let x: u64 = 600851475143;

    'main: for i in 2..x {
        if x % i == 0 {
            let y = x / i;

            for j in 2..y {
                if y % j == 0 {
                    continue 'main;
                }
            }

            prime = y;
            break;
        }
    }

    println!("{}", prime)
}
