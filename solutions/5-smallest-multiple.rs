fn main() {
    let mut x = 0;

    let num = 'main: loop {
        x += 1;

        for i in 1..=20 {
            if x % i != 0 {
                continue 'main;
            }
        }

        break x;
    };

    println!("{}", num);
}