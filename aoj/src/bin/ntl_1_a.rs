use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    // ------------------------------------

    print!("{}:", n);

    if is_prime(n) {
        println!(" {}", n);
        return;
    }

    let mut current = n;

    {
        let mut i = 2;

        while i * i <= n {
            while current % i == 0 {
                print!(" {}", i);
                current /= i;
            }

            if current == 1 {
                break;
            }

            i += 1;
        }

        if current != 1 {
            print!(" {}", current);
        }
    }

    println!();
}

fn is_prime(x: usize) -> bool {
    if x == 2 {
        return true;
    }

    if x < 2 || x % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i = i + 2;
    }

    true
}
