use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut even_count = 0;
    let mut odd_count = 0;

    for &value in &a {
        if value % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }
    }

    if even_count < 2 && odd_count < 2 {
        println!("-1");
        return;
    }

    a.sort();
    a.reverse();

    let mut even_list = Vec::new();
    let mut odd_list = Vec::new();

    let mut candi_even = -1;
    let mut candi_odd = -1;

    for &value in &a {
        if value % 2 == 0 {
            even_list.push(value);
        } else {
            odd_list.push(value);
        }

        if even_list.len() == 2 {
            candi_even = even_list[0] as isize + even_list[1] as isize;
        }

        if odd_list.len() == 2 {
            candi_odd = odd_list[0] as isize + odd_list[1] as isize;
        }
    }

    println!("{}", candi_even.max(candi_odd));
}
