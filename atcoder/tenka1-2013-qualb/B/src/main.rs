use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
        l: usize,
    }

    let mut stack = Vec::new();
    let mut size = 0;

    for _ in 0..q {
        input! {
            order: String,
        }

        match order.as_str() {
            "Push" => {
                input! {
                    n: usize,
                    m: isize,
                }

                size += n;

                if size > l {
                    println!("FULL");
                    return;
                }

                if let Some((value, count)) = stack.pop() {
                    if value == m {
                        stack.push((m, n + count as usize));
                    } else {
                        stack.push((value, count));
                        stack.push((m, n));
                    }
                } else {
                    stack.push((m, n));
                }
            }
            "Pop" => {
                input! {
                    mut n: usize,
                }

                size -= n;

                while n > 0 {
                    if let Some((value, count)) = stack.pop() {
                        if count > n {
                            stack.push((value, count - n));
                            n = 0;
                        } else {
                            n -= count;
                        }
                    } else {
                        break;
                    }
                }

                if n > 0 {
                    println!("EMPTY");
                    return;
                }
            }
            "Top" => {
                if let Some((value, count)) = stack.pop() {
                    println!("{}", value);

                    stack.push((value, count));
                } else {
                    println!("EMPTY");
                    return;
                }
            }
            _ => {
                println!("{}", size);
            }
        }
    }

    println!("SAFE");
}
