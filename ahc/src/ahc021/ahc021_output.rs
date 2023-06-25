// use itertools::Itertools;

use super::ahc021_input::Ahc021Input;

pub struct Ahc021Output {
    pub k: usize,
    pub x_y: Vec<(usize, usize, usize, usize)>,
}

impl Ahc021Output {
    pub fn print(&self, _input: &Ahc021Input) {
        println!("{}", self.k);

        for i in 0..self.k {
            let (x_0, y_0, x_1, y_1) = self.x_y[i];

            println!("{} {} {} {}", x_0, y_0, x_1, y_1);
        }
    }

    pub fn add(&mut self, x_0: usize, y_0: usize, x_1: usize, y_1: usize) {
        self.k += 1;
        self.x_y.push((x_0, y_0, x_1, y_1));
    }

    pub fn score(&self) -> usize {
        10
    }
}
