use procon_library_rs::geometry::point2f::Point2f;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        x_y: [Point2f; n],
    }

    let mut has_light = HashSet::new();

    for x in a {
        has_light.insert(x);
    }

    // 明かりを持ってない人と明かりを持ってない人の最短距離の最大

    let mut result = 0.;

    for dark in 0..n {
        if has_light.contains(&dark) {
            continue;
        }

        let mut score = None;

        for light in 0..n {
            if !has_light.contains(&light) {
                continue;
            }

            let point_light = x_y[light];
            let point_dark = x_y[dark];

            let len = point_light.euclidean_distance(point_dark);

            if let Some(value) = score {
                if len < value {
                    score = Some(len);
                }
            } else {
                score = Some(len);
            }
        }

        if score.unwrap() > result {
            result = score.unwrap();
        }
    }

    println!("{}", result);
}
