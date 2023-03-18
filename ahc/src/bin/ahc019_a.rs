use ahc::ahc019::{action_only_cubic::only_cubic, ahc019_input::Ahc019Input};
use proconio::{input, marker::Chars};

///
/// cargo equip --exclude-atcoder-crates --minify libs --remove docs --remove comments --bin ahc019_a | pbcopy
///

fn main() {
    input! {
        d: usize,
        f_1: [Chars; d],
        r_1: [Chars; d],
        f_2: [Chars; d],
        r_2: [Chars; d],
    }

    let input = Ahc019Input {
        d,
        f_1,
        r_1,
        f_2,
        r_2,
    };

    let result = only_cubic(&input);

    result.print(&input);
}
