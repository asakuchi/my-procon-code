use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if (s.rfind('B').unwrap() - s.find('B').unwrap()) % 2 != 0
        && s.find('R').unwrap() < s.find('K').unwrap()
        && s.find('K').unwrap() < s.rfind('R').unwrap()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
