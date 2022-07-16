use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q],
    }

    let mut balls = vec![0; n];
    let mut index = vec![0; n];

    for i in 0..n {
        balls[i] = i;
        index[i] = i;
    }

    for &num in x.iter() {
        let ball_index = index[num];
        let ball_value = balls[ball_index];

        let next_index = if ball_index == n - 1 {
            ball_index - 1
        } else {
            ball_index + 1
        };
        let next_value = balls[next_index];

        balls[next_index] = ball_value;
        balls[ball_index] = next_value;

        index[ball_value] = next_index;
        index[next_value] = ball_index;
    }

    for &ball in balls.iter() {
        println!("{}", ball + 1);
    }
}
