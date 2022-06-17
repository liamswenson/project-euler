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