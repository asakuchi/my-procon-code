fn main() {
    let mut tetra = Vec::new();

    for i in 1..1_000_000 {
        let value = i * (i + 1) * (i + 2) / 6;

        if value > 1_000_000 {
            break;
        }

        tetra.push(value);
    }

    // 180個程度
    // println!("len {}", tetra.len());

    let mut pollock = vec![1_000_000_000; 1_000_000];
    let mut pollock_odd = vec![1_000_000_000; 1_000_000];

    for &i in tetra.iter() {
        pollock[i] = 1;

        if i % 2 == 1 {
            pollock_odd[i] = 1;
        }
    }

    for i in 1..1_000_000 {
        for &j in tetra.iter() {
            if i + j >= 1_000_000 {
                break;
            }

            pollock[i + j] = pollock[i + j].min(pollock[i] + 1);

            if j % 2 == 1 {
                pollock_odd[i + j] = pollock_odd[i + j].min(pollock_odd[i] + 1);
            }
        }
    }

    loop {
        let n: usize = input_value();

        if n == 0 {
            break;
        }

        println!("{} {}", pollock[n], pollock_odd[n]);
    }
}

fn input_value<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n = buf.parse().unwrap();

    n
}
