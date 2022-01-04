// -*- coding:utf-8-unix -*-

// use proconio::input;

fn main() {
    let list = [[1, 2, 3], [4, 5, 6]];
    // assert_eq!(hash.values().max(), Some(&2));

    for line in list.iter() {
        for value in line {
            println!("{}", value);
        }
    }

    let list = [1, 2, 3];

    for value in list.iter() {
        println!("{}", value);
    }

    println!("{:?}", list);

    println!("end.");
}
