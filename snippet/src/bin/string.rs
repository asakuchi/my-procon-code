fn main() {
    let a = "hello";
    let b = "hello world";

    println!("{}", contain(a, b));
}

const B: u128 = 1902000031;
// const B2: u128 = 1999487509;

///
/// ローリングハッシュ
///
fn contain(a: &str, b: &str) -> bool {
    let a: Vec<_> = a.bytes().collect();
    let b: Vec<_> = b.bytes().collect();

    let a_length = a.len();
    let b_length = b.len();

    if a_length > b_length {
        return false;
    }

    let mut t = 1u128;

    for _ in 0..a_length {
        // t *= B;
        t = t.wrapping_mul(B);
    }

    let mut a_h = 0u128;
    let mut b_h = 0u128;

    for i in 0..a_length {
        a_h = a_h.wrapping_mul(B).wrapping_add(a[i].into());
    }

    for i in 0..a_length {
        b_h = b_h.wrapping_mul(B).wrapping_add(b[i].into());
    }

    {
        let mut i = 0;

        while i + a_length <= b_length {
            if a_h == b_h {
                return true;
            }

            if i + a_length < b_length {
                b_h = b_h
                    .wrapping_mul(B)
                    .wrapping_add(b[i + a_length].into())
                    .wrapping_sub((b[i] as u128).wrapping_mul(t));
            }

            i += 1;
        }
    }

    false
}
