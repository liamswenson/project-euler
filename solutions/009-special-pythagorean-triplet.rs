// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a2 + b2 = c2

// For example, 32 + 42 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let mut c = 1;

    let product = 'main: loop {
        for b in 0..c {
            for a in 0..b {
                if a * a + b * b == c * c {
                    if a + b + c == 1000 {
                        break 'main a * b * c;
                    };
                }
            }
        }

        c += 1;
    };

    println!("{}", product);
}