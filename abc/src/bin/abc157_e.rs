use ac_library_rs::{Additive, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }

    let mut list = Vec::new();

    for _ in 0..26 {
        list.push(Segtree::<Additive<usize>>::new(n + 1));
    }

    for i in 0..n {
        let c = s[i];
        let index = c as usize - 'a' as usize;

        let past = list[index].get(i);
        list[index].set(i, past + 1);
    }

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                i: Usize1,
                c: char,
            }

            let old_c = s[i];
            s[i] = c;

            // 減らす
            let index = old_c as usize - 'a' as usize;
            let past = list[index].get(i);
            list[index].set(i, past - 1);

            // 増やす
            let index = c as usize - 'a' as usize;
            let past = list[index].get(i);
            list[index].set(i, past + 1);
        } else {
            input! {
                l: Usize1,
                r: usize,
            }

            let mut count = 0;

            for c in 'a' as usize..='z' as usize {
                let index = c - 'a' as usize;

                if list[index].prod(l, r) > 0 {
                    count += 1;
                }
            }

            println!("{}", count);
        }
    }
}
