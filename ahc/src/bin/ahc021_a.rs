// use ahc::ahc021::action::block_solution::block_solution;
// use ahc::ahc021::action::multi_block_solution::multi_block_solution;
use ahc::ahc021::action::priority_multi_block_solution::priority_multi_block_solution;
use ahc::ahc021::ahc021_input::Ahc021Input;
use proconio::input;

///
/// cargo equip --exclude-atcoder-crates --minify libs --remove docs --remove comments --bin ahc021_a | pbcopy
///

fn main() {
    let n = 30;

    let mut pyramid = Vec::new();

    for i in 0..n {
        input! {
            b: [usize; i + 1],
        }

        pyramid.push(b);
    }

    // println!("{:?}", pyramid);

    let input = Ahc021Input { n, pyramid };

    // let result = first_solution(&input);
    // let result = block_solution(&input);
    // let result = multi_block_solution(&input);
    let result = priority_multi_block_solution(&input);

    result.print(&input);
}
