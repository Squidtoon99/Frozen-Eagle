use std::io::stdin;

fn main() {
    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    let t: u16 = line.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();

        stdin().read_line(&mut line).unwrap();

        let mut v: Vec<u64> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        if let [_limit, f2, f4, fn2, _fn4] = v.as_slice()[..] {
            let f1 = f4 - f2 * 2;

            // now that we have f1 and f2 we can generate the rest of the Lucas numbers
            let mut sum = f1 + f2;

            let mut count = 0;
            let mut n1 = f1;
            let mut n2 = f2;
            while n1 <= fn2 {
                let nth = n1 + n2;
                sum += nth;
                n1 = n2;
                n2 = nth;
                count += 1;
            }
            println!("{}", sum);
        }
    }
}
