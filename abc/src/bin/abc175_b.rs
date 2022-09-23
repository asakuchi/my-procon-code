use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }

    let mut result = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let mut edge = vec![l[i], l[j], l[k]];
                edge.sort();
                edge.dedup();

                if edge.len() != 3 {
                    continue;
                }

                if edge[0] + edge[1] > edge[2] {
                    result += 1;

                    // println!("{} {} {}", i, j, k);
                }
            }
        }
    }

    println!("{}", result);
}
