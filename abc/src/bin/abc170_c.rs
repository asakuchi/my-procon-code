use proconio::input;

fn main() {
    input! {
        x: usize,
        n: usize,
        p: [usize; n],
    }

    let mut list = vec![false; 102];

    for &num in &p {
        list[num] = true;
    }

    let mut diff = 0;

    loop {
        if !list[x - diff] {
            println!("{}", x - diff);
            return;
        }

        if !list[x + diff] {
            println!("{}", x + diff);
            return;
        }

        diff += 1;
    }
}
