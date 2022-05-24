use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: [[Usize1]; m],
        p: [usize; m],
    }

    let mut result = 0;

    'mask_loop: for mask in 0..1 << n {
        for light in 0..m {
            let mut on_count = 0;

            for &switch in k[light].iter() {
                if mask & 1 << switch > 0 {
                    on_count += 1;
                }
            }

            if on_count % 2 != p[light] {
                continue 'mask_loop;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
