use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let t = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();

        let mut index = 0;
        's: for i in 1..=n.len() {
            for j in 1..i {
                index += 1;
                if n.chars().rev().nth(i).unwrap() == '1' && n.chars().rev().nth(j).unwrap() == '1' {
                    println!("{}", index);
                    break 's;
                }
            }
        }
    }
}
