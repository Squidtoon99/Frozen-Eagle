use std::collections::HashMap;
use std::io::stdin;

type Interval = Vec<u64>;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    let t = line.trim().parse().unwrap();

    for _ in 0..t {
        line = String::new();
        stdin().read_line(&mut line).unwrap();
        let n = line.trim().parse().unwrap();

        let mut intervals = Vec::new();

        for _ in 0..n {
            line = String::new();
            stdin().read_line(&mut line).unwrap();

            let interval: Interval = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();

            intervals.push(interval);
        }

        let mut interval_relations: HashMap<&Interval, Vec<Interval>> = HashMap::new();

        for interval in &intervals {
            interval_relations.insert(interval, intervals.iter().filter(|x| x[0] > interval[0] && x[1] < interval[1]).collect());
        }
        println!("done: ");
        for relation in interval_relations {
            println!("{:#?}: {:#?}", relation.0, relation.1);
        }
    }
}
