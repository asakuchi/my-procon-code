fn main() {
    println!("{}", combination(10, 2));
}

fn combination(n: usize, k: usize) -> usize {
    let mut result = 1;

    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}
