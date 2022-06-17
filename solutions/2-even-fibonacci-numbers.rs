fn main() {
    let mut sum = 0;

    let mut x = 2;
    let mut y = 1;

    while x < 4000000 {
        if x % 2 == 0 {
            sum += x;
        }

        x += y;
        y = x - y;
    }

    println!("{}", sum);
}