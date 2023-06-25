use ahc::ahc019::action::only_cubic_outside::only_cubic_outside;
use ahc::ahc019::ahc019_input::Ahc019Input;
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

    // 全て 1*1*1 のブロックを敷き詰める
    // let result = only_cubic(&input);

    // 外側だけ、1*1*1 のブロックを敷き詰める
    let result = only_cubic_outside(&input);

    result.print(&input);
}
