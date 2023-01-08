use proconio::input;
use proconio::marker::Bytes;

const B: u128 = 1902000031;
const B2: u128 = 1999487509;

fn main() {
    input! {
        n: usize,
        t: Bytes,
    }

    // B の n - 1 乗
    let mut b_pow_n = 1u128;
    let mut b_2_pow_n = 1u128;

    for _ in 0..n - 1 {
        b_pow_n = b_pow_n.wrapping_mul(B);
        b_2_pow_n = b_2_pow_n.wrapping_mul(B2);
    }

    // S を反転した文字列のハッシュ
    let mut reverse_hash = 0u128;
    let mut reverse_hash_b2 = 0u128;
    // S の先頭i文字、S の末尾n-i文字のハッシュ
    let mut split_hash = 0u128;
    let mut split_hash_b2 = 0u128;

    // 反転した文字列
    // i 〜 i + n
    for i in (n..n * 2).rev() {
        reverse_hash = reverse_hash.wrapping_mul(B).wrapping_add(t[i].into());
        reverse_hash_b2 = reverse_hash_b2.wrapping_mul(B2).wrapping_add(t[i].into());
    }

    // 先頭i文字
    // 0 ~ i
    // 末尾 n-i 文字
    // n + i 〜 2 * n
    for i in 0..n {
        split_hash = split_hash.wrapping_mul(B).wrapping_add(t[i].into());
        split_hash_b2 = split_hash_b2.wrapping_mul(B2).wrapping_add(t[i].into());
    }

    if reverse_hash == split_hash && reverse_hash_b2 == split_hash_b2 {
        // OK
        let i = n;
        let text =
            String::from_utf8(t[i..n + i].iter().rev().map(|&x| x).collect::<Vec<_>>()).unwrap();
        println!("{}", text);
        println!("{}", i);
        return;
    }

    let mut b_pow_n_for_split = 1u128;
    let mut b_2_pow_n_for_split = 1u128;

    // i == n を満たすなら i == 0 を満たすので考慮しなくて良い
    for i in (1..n).rev() {
        reverse_hash = reverse_hash
            .wrapping_sub((t[i + n] as u128).wrapping_mul(b_pow_n))
            .wrapping_mul(B)
            .wrapping_add(t[i].into());

        reverse_hash_b2 = reverse_hash_b2
            .wrapping_sub((t[i + n] as u128).wrapping_mul(b_2_pow_n))
            .wrapping_mul(B2)
            .wrapping_add(t[i].into());

        split_hash = split_hash.wrapping_sub((t[i] as u128).wrapping_mul(b_pow_n_for_split));
        split_hash = split_hash.wrapping_add((t[i + n] as u128).wrapping_mul(b_pow_n_for_split));

        split_hash_b2 =
            split_hash_b2.wrapping_sub((t[i] as u128).wrapping_mul(b_2_pow_n_for_split));
        split_hash_b2 =
            split_hash_b2.wrapping_add((t[i + n] as u128).wrapping_mul(b_2_pow_n_for_split));

        b_pow_n_for_split = b_pow_n_for_split.wrapping_mul(B);
        b_2_pow_n_for_split = b_2_pow_n_for_split.wrapping_mul(B2);

        if reverse_hash == split_hash && reverse_hash_b2 == split_hash_b2 {
            // OK
            let text = String::from_utf8(t[i..n + i].iter().rev().map(|&x| x).collect::<Vec<_>>())
                .unwrap();
            println!("{}", text);
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
