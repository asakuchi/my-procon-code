use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut list = vec![vec![0; n]; n];

    let mut counter = 1;

    for i in 0..n {
        for j in 0..n {
            if j % 2 == 0 {
                list[i][j] = counter;
                counter += 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if j % 2 == 1 {
                list[i][j] = counter;
                counter += 1;
            }
        }
    }

    for i in 0..n {
        let line = list[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", line);
    }
}
