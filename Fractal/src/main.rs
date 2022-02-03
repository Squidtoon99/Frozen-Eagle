use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("Could not read line");

    let t = line.trim().parse().expect("Could not get test cases");

    for _ in 0..t {
        let mut line = String::new();

        stdin().read_line(&mut line).expect("Could not read line");

        let mut items: Vec<&str> = line.trim().split_ascii_whitespace().map(|x| x.trim()).collect();

        let mut seed = items.remove(items.len() - 1).to_string();
        let mut keys = HashMap::new();
        if let [n, k, r, c] = &items.into_iter().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()[..] {
            // Getting the keys necessary for input
            for _ in 0..*k {
                let mut key = String::new();
                stdin().read_line(&mut key).unwrap();
                key = key.trim().to_string();
                let mut rows = Vec::new();
                for _ in 0..*r {
                    let mut row = String::new();
                    stdin().read_line(&mut row).unwrap();
                    row = row.trim().to_string();
                    rows.push(row);
                }

                keys.insert(key, rows);
            }
            let mut output = String::new();
            for _ in 0..*n {
                for row in seed.lines() {
                    for row_index in 0..*r {
                        for char in row.chars().map(|x| x.to_string()) {
                            let y = char.repeat(*c);
                            let key: &str = match keys.get(&char.to_string()) {
                                Some(v) => {
                                    v.iter().nth(row_index).unwrap()
                                }
                                None => {
                                    &y
                                }
                            };

                            output += &key;
                        }
                        output += "\n";
                    }
                }
                seed = output.clone();
                output = String::new();
            }
            println!("{}", seed.trim_end());
        }
    }
}
