use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: u32,
        a: [usize; 2usize.pow(n)],
    }

    let mut player: Vec<_> = a.iter().enumerate().collect();

    while player.len() > 2 {
        player.sort();

        let size = player.len();
        let mut list = Vec::new();

        for i in (0..size).step_by(2) {
            if player[i].1 > player[i + 1].1 {
                list.push(player[i]);
            } else {
                list.push(player[i + 1]);
            }
        }

        player = list;
    }

    if player[0].1 > player[1].1 {
        println!("{}", player[1].0 + 1);
    } else {
        println!("{}", player[0].0 + 1);
    }
}
