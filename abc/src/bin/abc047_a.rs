use proconio::input;

fn main() {
    input! {
        mut candy: [usize; 3],
    }

    candy.sort();

    if candy[0] + candy[1] == candy[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
