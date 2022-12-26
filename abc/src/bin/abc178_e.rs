//!
//! マンハッタン距離の最大値
//! https://fukubutyo.web.fc2.com/Manhattan.html
//!

use proconio::{derive_readable, input};

#[derive_readable]
#[derive(Copy, Clone, PartialEq, Debug)]
struct IsizePoint2 {
    x: isize,
    y: isize,
}

impl IsizePoint2 {
    /// 原点、零ベクトル
    #[allow(dead_code)]
    const ZERO: IsizePoint2 = IsizePoint2 { x: 0, y: 0 };

    /// 単位ベクトル
    #[allow(dead_code)]
    const ONE: IsizePoint2 = IsizePoint2 { x: 1, y: 1 };

    /// マンハッタン距離
    /// 2点間の距離
    #[allow(dead_code)]
    fn manhattan_distance(&self, rhs: IsizePoint2) -> isize {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    ///
    /// 45度回転して√2倍する
    ///
    /// 『マンハッタン距離は45度回転』
    /// 大きさが変わっているので注意
    /// https://kagamiz.hatenablog.com/entry/2014/12/21/213931
    ///
    /// 変換後の座標を x', y' とすると、
    /// a(x_a, y_a) と b(x_b, y_b) のマンハッタン距離は
    /// max(|x'_a - x'_b|, |y'_a, y'_b|)
    ///
    #[allow(dead_code)]
    fn rotate_45(&self) -> IsizePoint2 {
        IsizePoint2 {
            x: self.x - self.y,
            y: self.x + self.y,
        }
    }

    fn max_manhattan_distance(list: &Vec<IsizePoint2>) -> isize {
        let x_y_45 = list.iter().map(|p| p.rotate_45());

        let max_x: isize = x_y_45.clone().map(|p| p.x).max().unwrap();
        let min_x: isize = x_y_45.clone().map(|p| p.x).min().unwrap();
        let max_y: isize = x_y_45.clone().map(|p| p.y).max().unwrap();
        let min_y: isize = x_y_45.clone().map(|p| p.y).min().unwrap();

        let result = (max_x - min_x).max(max_y - min_y);

        result
    }
}

fn main() {
    input! {
        n: usize,
        x_y: [IsizePoint2; n],
    }

    let result = IsizePoint2::max_manhattan_distance(&x_y);

    println!("{}", result);
}
