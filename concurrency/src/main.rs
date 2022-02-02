use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).ok().expect("read error");

    let amount: usize = line.trim().parse().ok().expect("parse error");

    for _ in 0..amount {
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok().expect("read error");
        let n: u32 = line.trim().parse().ok().expect("parse error");

        let mut permutations: Vec<(u32, u32, u32)> = Vec::new();

        for a in 1..n {
            for b in 1..n {
                for c in 1..n {
                    if a * b * c == (n - a) * (n - b) * (n - c) {
                        permutations.push((a, b, c));
                    }
                }
            }
        }

        println!("{}", permutations.len());
    }
}
