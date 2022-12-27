use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let mut imos = vec![0; 1_000_002];

    for (a, b) in a_b {
        imos[a] += 1;
        imos[b + 1] -= 1;
    }

    // println!("{:?}", &imos[..10]);

    let mut list = vec![0; 1_000_001];

    list[0] = imos[0];

    let mut result = list[0];

    for color in 1..=1_000_000 {
        list[color] += imos[color] + list[color - 1];

        result = result.max(list[color]);
    }

    // println!("{:?}", &list[..10]);

    println!("{}", result);
}
