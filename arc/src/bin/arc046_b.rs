// use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    if a == b {
        if n % (a + 1) != 0 {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else if a > b {
        println!("Takahashi");
    } else {
        if n <= a {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }

    // // 実験
    // for i in 0..=20 {
    //     println!("taka i {} g {}", i, grundy(a, b, i, true));
    // }

    // for i in 0..=20 {
    //     println!("aoki i {} g {}", i, grundy(a, b, i, false));
    // }
}

// fn grundy(a: usize, b: usize, x: usize, taka_turn: bool) -> usize {
//     if x == 0 {
//         return 0;
//     }

//     let mut set = HashSet::new();

//     if taka_turn {
//         for i in 1..=a {
//             if x >= i {
//                 set.insert(grundy(a, b, x - i, false));
//             }
//         }
//     } else {
//         for i in 1..=b {
//             if x >= i {
//                 set.insert(grundy(a, b, x - i, true));
//             }
//         }
//     }

//     let mut g = 0;

//     while set.contains(&g) {
//         g += 1;
//     }

//     g
// }
