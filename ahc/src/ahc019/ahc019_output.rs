use itertools::Itertools;

use super::ahc019_input::Ahc019Input;

pub struct Ahc019Output {
    /// ブロックの番号
    pub block_number_1: Vec<Vec<Vec<Option<usize>>>>,
    pub block_number_2: Vec<Vec<Vec<Option<usize>>>>,
}

impl Ahc019Output {
    pub fn print(&self, input: &Ahc019Input) {
        let mut max_block = 1;

        let mut result_1 = Vec::new();
        let mut result_2 = Vec::new();

        for x in 0..input.d {
            for y in 0..input.d {
                for z in 0..input.d {
                    if let Some(block) = self.block_number_1[x][y][z] {
                        result_1.push(block);

                        max_block = max_block.max(block);
                    } else {
                        // どのブロックもその領域を占めていない
                        result_1.push(0);
                    }
                }
            }
        }

        for x in 0..input.d {
            for y in 0..input.d {
                for z in 0..input.d {
                    if let Some(block) = self.block_number_2[x][y][z] {
                        result_2.push(block);

                        max_block = max_block.max(block);
                    } else {
                        // どのブロックもその領域を占めていない
                        result_2.push(0);
                    }
                }
            }
        }

        println!("{}", max_block);
        println!("{}", result_1.iter().format(" "));
        println!("{}", result_2.iter().format(" "));
    }
}
