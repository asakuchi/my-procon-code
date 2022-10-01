use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut has_book = vec![false; n + 1];

    let mut others = 0;
    let mut books = VecDeque::new();

    for i in 0..n {
        let book = a[i];

        if book > n {
            others += 1;
        } else if has_book[book] {
            others += 1;
        } else {
            has_book[book] = true;
            books.push_back(book);
        }
    }

    for _ in 0..others {
        // 十分大きな値
        books.push_back(1_000_000_001);
    }

    let mut reading = 0;

    while let Some(book) = books.pop_front() {
        if book == reading + 1 {
            reading += 1;
        } else {
            books.push_front(book);

            if let (Some(_other_book_1), Some(_other_book_2)) = (books.pop_back(), books.pop_back())
            {
                reading += 1;
            } else {
                break;
            }
        }
    }

    println!("{}", reading);
}
