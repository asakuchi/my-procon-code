fn main() {
    let (n, m) = input_tuple();
    let p = input_vec();

    let mut tree = BinaryIndexedTree::new(n);

    // 転倒数
    let mut total = 0;

    for i in 0..n {
        let x: usize = p[i];

        total += i as isize - tree.sum(x);
        tree.add(x, 1);
    }

    // println!("total {}", total);

    let m = m as isize;

    if total % m == 0 {
        println!("{}", total);
        return;
    }

    let candi_1 = (total / m + 1) * m;

    // println!("candi_1 {}", candi_1);

    if (candi_1 - total) % 2 == 0 {
        println!("{}", candi_1);
        return;
    }

    let candi_2 = (total / m + 2) * m;

    // println!("candi_2 {}", candi_2);

    if (candi_2 - total) % 2 == 0 {
        println!("{}", candi_2);
        return;
    }

    println!("-1");
}

// fn gcd(a: isize, b: isize) -> isize {
//     if b == 0 {
//         return a;
//     } else {
//         return gcd(b, a % b);
//     }
// }
// /*  lcm2 (a, b) : 2整数版
//     入力：整数 a, b
//     出力：aとbの最小公倍数
// */
// fn lcm2(a: isize, b: isize) -> isize {
//     let d = gcd(a, b);
//     return a / d * b;
// }

struct BinaryIndexedTree {
    n: usize,
    bit: Vec<isize>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> BinaryIndexedTree {
        let bit = vec![0; n + 1];

        BinaryIndexedTree { n: n + 1, bit }
    }

    fn add(&mut self, i: usize, x: isize) {
        let mut index = i as isize;

        while (index as usize) < self.n {
            self.bit[index as usize] += x;

            index += index & -index;
        }
    }

    fn sum(&self, i: usize) -> isize {
        let mut s = 0;

        let mut index = i as isize;

        while index > 0 {
            s += self.bit[index as usize];

            index -= index & -index;
        }

        s
    }
}

fn input_tuple<T>() -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let line = iter.map(|x| x.parse().unwrap()).collect();

    line
}
