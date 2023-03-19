use crate::ahc019::{ahc019_input::Ahc019Input, ahc019_output::Ahc019Output};

///
/// 全て 1*1*1 のブロックを敷き詰める
///
pub fn only_cubic(input: &Ahc019Input) -> Ahc019Output {
    let mut has_block_1 = vec![vec![vec![true; input.d]; input.d]; input.d];
    let mut has_block_2 = vec![vec![vec![true; input.d]; input.d]; input.d];

    for x in 0..input.d {
        for z in 0..input.d {
            if input.f_1[z][x] == '0' {
                for y in 0..input.d {
                    has_block_1[x][y][z] = false;
                }
            }

            if input.f_2[z][x] == '0' {
                for y in 0..input.d {
                    has_block_2[x][y][z] = false;
                }
            }
        }
    }

    for z in 0..input.d {
        for y in 0..input.d {
            if input.r_1[z][y] == '0' {
                for x in 0..input.d {
                    has_block_1[x][y][z] = false;
                }
            }

            if input.r_2[z][y] == '0' {
                for x in 0..input.d {
                    has_block_2[x][y][z] = false;
                }
            }
        }
    }

    let mut block = 0;

    let mut block_number_1 = vec![vec![vec![None; input.d]; input.d]; input.d];
    let mut block_number_2 = vec![vec![vec![None; input.d]; input.d]; input.d];

    for x in 0..input.d {
        for y in 0..input.d {
            for z in 0..input.d {
                if has_block_1[x][y][z] {
                    block += 1;
                    block_number_1[x][y][z] = Some(block);
                }
            }
        }
    }

    for x in 0..input.d {
        for y in 0..input.d {
            for z in 0..input.d {
                if has_block_2[x][y][z] {
                    block += 1;
                    block_number_2[x][y][z] = Some(block);
                }
            }
        }
    }

    Ahc019Output {
        block_number_1,
        block_number_2,
    }
}
