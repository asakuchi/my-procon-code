use proconio::input;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }

    let mut card = vec![vec![false; 3]; 3];

    for x in b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == x {
                    card[i][j] = true;
                }
            }
        }
    }

    for i in 0..3 {
        let mut ok = true;

        for j in 0..3 {
            if !card[i][j] {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    for j in 0..3 {
        let mut ok = true;

        for i in 0..3 {
            if !card[i][j] {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    {
        let mut ok = true;

        for i in 0..3 {
            if !card[i][i] {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    {
        let mut ok = true;

        for i in 0..3 {
            if !card[i][2 - i] {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
