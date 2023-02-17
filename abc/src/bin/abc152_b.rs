use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let text_1 = a.to_string().repeat(b);
    let text_2 = b.to_string().repeat(a);

    println!("{}", text_1.min(text_2));
}
