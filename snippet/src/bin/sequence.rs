fn main() {}

///
/// 等差数列の和
///
fn tousa_sum(a: usize, d: usize, n: usize) -> usize {
    (a * 2 + d * (n - 1)) * n / 2
}
