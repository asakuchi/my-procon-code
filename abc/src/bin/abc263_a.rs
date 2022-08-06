use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        cards: [usize; 5],
    }

    let mut list = vec![0; 14];

    for i in 0..5 {
        list[cards[i]] += 1;
    }

    let mut has_two = false;
    let mut has_tree = false;

    for i in 0..14 {
        if list[i] == 2 {
            has_two = true;
        }

        if list[i] == 3 {
            has_tree = true;
        }
    }

    if has_two && has_tree {
        println!("Yes");
    } else {
        println!("No");
    }
}
