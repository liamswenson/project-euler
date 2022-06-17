fn main() {
    let mut product = 0;

    for i in (1..1000).rev() {
        for j in (1..1000).rev() {
            let x = i * j;

            if x > product {
                let x_str = x.to_string();
                let x_str_rev: String = x_str.chars().rev().collect();

                if x_str == x_str_rev {
                    product = x;
                }
            }
        }
    }

    println!("{}", product);
}
