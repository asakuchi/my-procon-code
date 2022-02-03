fn main() {
    let mut high = 10u128.pow(9) + 1;
    let mut low = 0;
    let mut mid;

    loop {
        mid = (high + low) / 2;

        if mid == low {
            break;
        }

        if some_condition {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{}", mid);
}
