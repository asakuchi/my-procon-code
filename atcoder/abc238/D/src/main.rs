use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        t: usize,
        a_s: [(usize, usize); t],
    }

    MyStruct { a_s }.main();
}

struct MyStruct {
    a_s: Vec<(usize, usize)>,
}

impl MyStruct {
    fn main(&mut self) {
        for (a, s) in self.a_s.iter() {
            println!("question a:{:020b} s:{:020b}", a, s);
            let answer = self.dfs(*a, *s);

            println!("{}", if answer { "Yes" } else { "No" });
        }
    }

    fn dfs(&self, a: usize, s: usize) -> bool {
        println!("a:{:020b} s:{:020b}", a, s);

        if s == 0 {
            return a == 0;
        }

        for x in 0..2 {
            for y in 0..2 {
                if (x & y) != (a & 1) {
                    continue;
                }
                if x + y > s {
                    continue;
                }
                if (s - x - y) % 2 != 0 {
                    continue;
                }

                println!("x{} y{}", x, y);
                if self.dfs(a >> 1, (s - x - y) >> 1) {
                    return true;
                }
            }
        }

        false
    }
}
