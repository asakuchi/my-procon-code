//!
//! 幾何
//!

fn main() {
    let p1 = Point2(1., 1.);
    let p2 = Point2(2., 3.);

    println!("{:?}", p1 + p2);
    println!("{:?}", p1 - p2);
    println!("{:?}", p1 * 3.);

    println!("{}", cos_formula(3., 4., 0.5 * std::f64::consts::PI,));
    println!("{:?}", rotate(1., 2., 3., 4., 0.5 * std::f64::consts::PI));
}

const EPS: f64 = 1e-10;

/// 座標
#[derive(Copy, Clone, Debug)]
struct Point2(f64, f64);

impl std::ops::Add for Point2 {
    type Output = Point2;

    fn add(self, rhs: Self) -> Self {
        Point2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Self) -> Self {
        Point2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<f64> for Point2 {
    type Output = Point2;

    fn mul(self, k: f64) -> Self {
        Point2(self.0 * k, self.1 * k)
    }
}

enum CCW_PATTERN {
    COUNTER_CLOCKWISE,
    CLOCKWISE,
    ONLINE_BACK,
    ONLINE_FRONT,
    ON_SEGMENT,
}

impl Point2 {
    /// ノルム
    /// ベクトルの大きさ
    fn norm(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

    /// ノルム
    /// ベクトルの大きさ
    fn abs(&self) -> f64 {
        self.norm().abs()
    }

    /// マンハッタン距離
    /// 2点間の距離
    fn manhattan_distance(&self, rhs: Point2) -> f64 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }

    /// ユークリッド距離
    /// 2点間の距離
    fn euclidean_distance(&self, rhs: Point2) -> f64 {
        ((self.0 - rhs.0).powf(2.) + (self.1 - rhs.1).powf(2.)).sqrt()
    }

    /// 内積
    fn dot(&self, rhs: Point2) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    /// 外積
    fn cross(&self, rhs: Point2) -> f64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    /// 2つのベクトルが直交するか
    ///
    /// 内積0なら直交している
    fn is_orthogonal(&self, rhs: Point2) -> bool {
        self.dot(rhs).abs() < EPS
    }

    /// 2つのベクトルが平行か
    ///
    /// 外積0なら平行
    fn is_parallel(&self, rhs: Point2) -> bool {
        self.cross(rhs).abs() < EPS
    }

    ///
    /// ベクトルa,bの位置関係
    ///
    fn counter_clockwise(p0: Point2, p1: Point2, p2: Point2) -> CCW_PATTERN {
        let a = p1 - p0;
        let b = p2 - p0;

        if a.cross(b) > EPS {
            return CCW_PATTERN::COUNTER_CLOCKWISE;
        }

        if a.cross(b) < -EPS {
            return CCW_PATTERN::CLOCKWISE;
        }

        if a.dot(b) < -EPS {
            return CCW_PATTERN::ONLINE_BACK;
        }

        if a.norm() < b.norm() {
            return CCW_PATTERN::ONLINE_FRONT;
        }

        CCW_PATTERN::ON_SEGMENT
    }
}

/// 線分
struct Segment2(Point2, Point2);

impl Segment2 {
    /// 射影
    ///
    /// 点p から線分に垂線を引いた時の交点
    fn project(&self, p: Point2) -> Point2 {
        let base = self.1 - self.0;
        let r = (p - self.0).dot(base) / base.norm();
        self.0 + base * r
    }

    /// 反射
    ///
    /// 線分を対称軸とした点pの線対称の点
    fn reflect(&self, p: Point2) -> Point2 {
        p + (self.project(p) - p) * 2.
    }
}

/// 直線
struct Line2(Segment2);

/// 円
struct Circle {
    center: Point2,
    radius: f64,
}

/// 多角形
struct Polygon(Vec<Point2>);

///
/// 余弦定理
///
/// ```
/// assert_eq!(cos_formula(3., 4., 0.5 * std::f64::consts::PI,), 5);
/// ```
///
fn cos_formula(a: f64, b: f64, c_angle_radian: f64) -> f64 {
    (a * a + b * b - 2. * a * b * c_angle_radian.cos()).sqrt()
}

///
/// 任意点周りの回転移動（アフィン変換）
/// https://imagingsolution.net/math/rotate-around-point/
///
fn rotate(x: f64, y: f64, center_x: f64, center_y: f64, angle_radian: f64) -> (f64, f64) {
    let rotated_x = x * angle_radian.cos() - y * angle_radian.sin() + center_x
        - center_x * angle_radian.cos()
        + center_y * angle_radian.sin();
    let rotated_y = x * angle_radian.sin() + y * angle_radian.cos() + center_y
        - center_x * angle_radian.sin()
        - center_y * angle_radian.cos();

    (rotated_x, rotated_y)
}
