use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a_b: [(usize, usize); m],
    }

    let mut buttery = n;

    let mut leave_time = 0;

    // println!("start {}", buttery);

    for &(a, b) in &a_b {
        if a - leave_time >= buttery {
            println!("No");
            return;
        }

        buttery -= a - leave_time;

        // println!("cafe start {}", buttery);

        buttery += b - a;

        if buttery > n {
            buttery = n;
        }

        // println!("cafe leave {}", buttery);

        leave_time = b;
    }

    // println!("home {} ? {}", buttery, t - leave_time);

    if t - leave_time >= buttery {
        println!("No");
        return;
    }

    println!("Yes");
}
