use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut list: Vec<(Option<usize>, Option<usize>)> = Vec::new();

    for _ in 0..n {
        // 前、次
        list.push((None, None));
    }

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }

                list[x].1 = Some(y);
                list[y].0 = Some(x);
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }

                list[x].1 = None;
                list[y].0 = None;
            }
            _ => {
                input! {
                    x: Usize1,
                }

                let mut output = Vec::new();

                let mut current = x;

                // 前を探す
                while let Some(prev) = list[current].0 {
                    output.push(prev);
                    current = prev;
                }

                output.reverse();

                let mut current = x;
                output.push(current);

                // 後を探す
                while let Some(next) = list[current].1 {
                    output.push(next);
                    current = next;
                }

                let line = output
                    .iter()
                    .map(|num| (num + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(" ");

                println!("{} {}", output.len(), line);
            }
        }
    }
}
