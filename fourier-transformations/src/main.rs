use std::io;

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}


fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line);

    let t = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line);
        let n = line.trim().parse::<u32>().unwrap();
        let best_radix = (2..=36).max_by_key(|&radix| {
            format_radix(n, radix).matches("4").count()
        }).expect("Expected at least one radix");
        println!("{} {}", best_radix, format_radix(n, best_radix));
    }
}
