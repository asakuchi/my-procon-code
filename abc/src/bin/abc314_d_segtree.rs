use proconio::{input, marker::Chars};

use ac_library::{LazySegtree, MapMonoid, Monoid};

#[derive(Clone)]
struct Counter {
    lower: usize,
    upper: usize,
}

#[derive(Clone)]
enum QueryType {
    Replace,
    Lower,
    Upper,
}

struct CharSet;

impl Monoid for CharSet {
    ///
    /// モノイドの型
    ///
    type S = Counter;

    ///
    /// 単位元
    ///
    fn identity() -> Self::S {
        Counter { lower: 0, upper: 0 }
    }

    ///
    /// 二項演算
    ///
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        // 後勝
        Counter {
            lower: a.lower + b.lower,
            upper: a.upper + b.upper,
        }
    }
}

struct MapCharSet;

impl MapMonoid for MapCharSet {
    type M = CharSet;
    /// 写像の型
    type F = QueryType;

    ///
    /// 恒等写像
    /// 全ての`a`に対して`mapping(id, a) = a`となるもの
    ///
    fn identity_map() -> Self::F {
        QueryType::Replace
    }

    ///
    /// f(x) を返す関数
    ///
    /// dataの値`x`に対して作用させる関数
    ///
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        match f {
            QueryType::Replace => x.clone(),
            QueryType::Lower => Counter {
                lower: x.lower + x.upper,
                upper: 0,
            },
            QueryType::Upper => Counter {
                lower: 0,
                upper: x.lower + x.upper,
            },
        }
    }

    ///
    /// f∘g を返す関数
    ///
    /// `g` がこれまでの操作、`f` が後に追加する操作で、
    ///「その2つの操作を順に行うようなひとまとめの操作（合成写像）」を返す
    ///
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        match f {
            QueryType::Replace => g.clone(),
            _ => f.clone(),
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        t_x_c: [(usize, usize, char); q],
    }

    let initial: Vec<_> = s
        .iter()
        .map(|c| {
            if c.is_ascii_uppercase() {
                Counter { lower: 0, upper: 1 }
            } else {
                Counter { lower: 1, upper: 0 }
            }
        })
        .collect();

    let mut tree = LazySegtree::<MapCharSet>::from(initial);

    for i in 0..q {
        let (t, x, c) = t_x_c[i];

        if t == 1 {
            tree.apply(
                x - 1,
                if c.is_ascii_uppercase() {
                    QueryType::Upper
                } else {
                    QueryType::Lower
                },
            );

            s[x - 1] = c;
        } else if t == 2 {
            tree.apply_range(0..n, QueryType::Lower);
        } else if t == 3 {
            tree.apply_range(0..n, QueryType::Upper);
        }
    }

    let mut result = Vec::new();

    for i in 0..n {
        if tree.get(i).upper > 0 {
            result.push(s[i].to_ascii_uppercase());
        } else {
            result.push(s[i].to_ascii_lowercase());
        }
    }

    let text: String = result.iter().collect();
    println!("{}", text);
}
