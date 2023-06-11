use proconio::input;

fn main() {
    input! {
        p: char,
        q: char,
    }

    let list = vec![0, 3, 4, 8, 9, 14, 23];

    let left = (p as u8 - 'A' as u8).min(q as u8 - 'A' as u8);
    let right = (p as u8 - 'A' as u8).max(q as u8 - 'A' as u8);

    println!("{}", list[right as usize] - list[left as usize]);
}
