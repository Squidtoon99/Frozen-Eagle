use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let t = line.trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        let mut b = dbg!(process(&mut line));

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let xy: Vec<f64> = dbg!(process(&mut line));

        let mut x = 0.0;
        let mut y = 0.0;
        for (i, item) in xy.iter().enumerate() {
            match i % 2 {
                0 => x += item * b.get(0).unwrap(),
                1 => y += item * b.remove(0),
                _ => unreachable!(),
            };
        }

        println!("{:.2} {:.2}", x, y);
    }
}

fn process(line: &mut String) -> Vec<f64> {
    line.trim().split_whitespace().map(|x| x.parse::<_>().unwrap()).collect::<_>()
}