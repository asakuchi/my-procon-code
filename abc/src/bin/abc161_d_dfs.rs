use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let mut lunlun_list = Vec::new();

    for x in 1..=9 {
        rec(x, 1, &mut lunlun_list);
    }

    lunlun_list.sort();

    println!("{}", lunlun_list[k - 1]);
}

fn rec(x: usize, keta: usize, lunlun_list: &mut Vec<usize>) {
    lunlun_list.push(x);

    if keta == 10 {
        return;
    }

    let tail = x % 10;

    if tail > 0 {
        rec(x * 10 + tail - 1, keta + 1, lunlun_list);
    }

    rec(x * 10 + tail, keta + 1, lunlun_list);

    if tail < 9 {
        rec(x * 10 + tail + 1, keta + 1, lunlun_list);
    }
}
