//!
//! 幾何
//!

fn main() {
    let p1 = Point2 { x: 1., y: 1. };
    let p2 = Point2 { x: 2., y: 3. };

    println!("{:?}", p1 + p2);
    println!("{:?}", p1 - p2);
    println!("{:?}", p1 * 3.);

    println!("{}", cos_formula(3., 4., 0.5 * std::f64::consts::PI,));
    // println!("{:?}", rotate(1., 2., 3., 4., 0.5 * std::f64::consts::PI));
    p1.rotate(Point2 { x: 0., y: 0. }, Degree(30.).to_radian());
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------
// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

#[allow(dead_code)]
const EPS: f64 = 1e-10;

/// 度(度数法)
// #[derive_readable]
#[derive(Copy, Clone, Debug)]
struct Degree(f64);

/// ラジアン(弧度法)
// #[derive_readable]
#[derive(Copy, Clone, Debug)]
struct Radian(f64);

/// 座標, ベクトル
// #[derive_readable]
#[derive(Copy, Clone, Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

/// 線分、直線
// #[derive_readable]
#[derive(Copy, Clone, Debug)]
struct Segment2(Point2, Point2);

/// 円
// #[derive_readable]
#[derive(Copy, Clone, Debug)]
struct Circle {
    center: Point2,
    radius: f64,
}

/// 多角形
// #[derive_readable]
#[derive(Clone, Debug)]
struct Polygon(Vec<Point2>);

impl Degree {
    #[allow(dead_code)]
    fn to_radian(&self) -> Radian {
        Radian(self.0 * std::f64::consts::PI / 180.)
    }
}

trait ToDegree {
    fn to_degree(&self) -> Degree;
}

impl ToDegree for f64 {
    fn to_degree(&self) -> Degree {
        Degree(*self)
    }
}

impl std::ops::Add for Degree {
    type Output = Degree;

    fn add(self, rhs: Self) -> Self {
        Degree(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Degree {
    type Output = Degree;

    fn sub(self, rhs: Self) -> Self {
        Degree(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Degree {
    type Output = Degree;

    fn mul(self, k: f64) -> Self {
        Degree(self.0 * k)
    }
}

impl std::ops::Div<f64> for Degree {
    type Output = Degree;

    fn div(self, k: f64) -> Self {
        Degree(self.0 / k)
    }
}

impl Radian {
    #[allow(dead_code)]
    fn to_degree(&self) -> Degree {
        Degree(self.0 * 180. / std::f64::consts::PI)
    }
}

trait ToRadian {
    fn to_radian(&self) -> Radian;
}

impl ToRadian for f64 {
    fn to_radian(&self) -> Radian {
        Radian(*self)
    }
}

impl std::ops::Add for Radian {
    type Output = Radian;

    fn add(self, rhs: Self) -> Self {
        Radian(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Radian {
    type Output = Radian;

    fn sub(self, rhs: Self) -> Self {
        Radian(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Radian {
    type Output = Radian;

    fn mul(self, k: f64) -> Self {
        Radian(self.0 * k)
    }
}

impl std::ops::Div<f64> for Radian {
    type Output = Radian;

    fn div(self, k: f64) -> Self {
        Radian(self.0 / k)
    }
}

impl std::ops::Add for Point2 {
    type Output = Point2;

    fn add(self, rhs: Self) -> Self {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Self) -> Self {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Point2 {
    type Output = Point2;

    fn mul(self, k: f64) -> Self {
        Point2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl std::ops::Div<f64> for Point2 {
    type Output = Point2;

    fn div(self, k: f64) -> Self {
        Point2 {
            x: self.x / k,
            y: self.y / k,
        }
    }
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum CcwPattern {
    CounterClockwise = 1,
    Clockwise = -1,
    OnlineBack = 2,
    OnlineFront = -2,
    OnSegment = 0,
}

impl Point2 {
    /// ノルム
    /// ベクトルの大きさ
    #[allow(dead_code)]
    fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// ノルム
    /// ベクトルの大きさ
    #[allow(dead_code)]
    fn abs(&self) -> f64 {
        self.norm().sqrt()
    }

    /// マンハッタン距離
    /// 2点間の距離
    #[allow(dead_code)]
    fn manhattan_distance(&self, rhs: Point2) -> f64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    /// ユークリッド距離
    /// 2点間の距離
    #[allow(dead_code)]
    fn euclidean_distance(&self, rhs: Point2) -> f64 {
        ((self.x - rhs.x).powf(2.) + (self.y - rhs.y).powf(2.)).sqrt()
    }

    /// 内積
    #[allow(dead_code)]
    fn dot(&self, rhs: Point2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// 外積
    #[allow(dead_code)]
    fn cross(&self, rhs: Point2) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// 2つのベクトルが直交するか
    ///
    /// 内積0なら直交している
    #[allow(dead_code)]
    fn is_orthogonal(&self, rhs: Point2) -> bool {
        self.dot(rhs).abs() < EPS
    }

    /// 2つのベクトルが平行か
    ///
    /// 外積0なら平行
    #[allow(dead_code)]
    fn is_parallel(&self, rhs: Point2) -> bool {
        self.cross(rhs).abs() < EPS
    }

    ///
    /// ベクトルa,bの位置関係
    ///
    #[allow(dead_code)]
    fn counter_clockwise(p0: Point2, p1: Point2, p2: Point2) -> CcwPattern {
        let a = p1 - p0;
        let b = p2 - p0;

        if a.cross(b) > EPS {
            return CcwPattern::CounterClockwise;
        }

        if a.cross(b) < -EPS {
            return CcwPattern::Clockwise;
        }

        if a.dot(b) < -EPS {
            return CcwPattern::OnlineBack;
        }

        if a.norm() < b.norm() {
            return CcwPattern::OnlineFront;
        }

        CcwPattern::OnSegment
    }

    /// 線分 p1p2 と線分 p3p4 の交差判定
    #[allow(dead_code)]
    fn intersect(p1: Point2, p2: Point2, p3: Point2, p4: Point2) -> bool {
        Self::counter_clockwise(p1, p2, p3) as i32 * Self::counter_clockwise(p1, p2, p4) as i32 <= 0
            && Self::counter_clockwise(p3, p4, p1) as i32
                * Self::counter_clockwise(p3, p4, p2) as i32
                <= 0
    }

    /// 角度
    #[allow(dead_code)]
    fn arg(&self) -> Radian {
        Radian(self.y.atan2(self.x))
    }

    /// 極座標から変換
    #[allow(dead_code)]
    fn poloar(r: f64, theta: Radian) -> Point2 {
        Point2 {
            x: theta.0.cos() * r,
            y: theta.0.sin() * r,
        }
    }

    /// 任意点周りの回転移動（アフィン変換）
    /// https://imagingsolution.net/math/rotate-around-point/
    #[allow(dead_code)]
    fn rotate(&self, center: Point2, angle: Radian) -> Point2 {
        Point2 {
            x: self.x * angle.0.cos() - self.y * angle.0.sin() + center.x
                - center.x * angle.0.cos()
                + center.y * angle.0.sin(),
            y: self.x * angle.0.sin() + self.y * angle.0.cos() + center.y
                - center.x * angle.0.sin()
                - center.y * angle.0.cos(),
        }
    }
}

impl Segment2 {
    /// 線分をベクトル表現
    #[allow(dead_code)]
    fn to_vector(&self) -> Point2 {
        self.1 - self.0
    }

    /// 線分をベクトル表現（逆向き）
    #[allow(dead_code)]
    fn to_reversed_vector(&self) -> Point2 {
        self.0 - self.1
    }

    /// 射影
    ///
    /// 点p から線分に垂線を引いた時の交点
    #[allow(dead_code)]
    fn project(&self, p: Point2) -> Point2 {
        let base = self.1 - self.0;
        let r = (p - self.0).dot(base) / base.norm();
        self.0 + base * r
    }

    /// 反射
    ///
    /// 線分を対称軸とした点pの線対称の点
    #[allow(dead_code)]
    fn reflect(&self, p: Point2) -> Point2 {
        p + (self.project(p) - p) * 2.
    }

    /// 線分 p1p2 と線分 p3p4 の交差判定
    #[allow(dead_code)]
    fn intersect(&self, rhs: Segment2) -> bool {
        Point2::intersect(self.0, self.1, rhs.0, rhs.1)
    }

    /// 線分と点pの距離
    #[allow(dead_code)]
    fn distance_from_point(&self, p: Point2) -> f64 {
        // 場合分けが必要

        if (self.1 - self.0).dot(p - self.0) < 0. {
            return (p - self.0).abs();
        }

        if (self.0 - self.1).dot(p - self.1) < 0. {
            return (p - self.1).abs();
        }

        self.distance_between_line_and_point(p)
    }

    /// 直線と点pの距離
    #[allow(dead_code)]
    fn distance_between_line_and_point(&self, p: Point2) -> f64 {
        let vector_a = self.1 - self.0;
        let vector_b = p - self.0;

        vector_a.cross(vector_b).abs() / vector_a.abs()
    }

    /// 線分 s1 と線分 s2 の距離
    #[allow(dead_code)]
    fn distance_from_segment(&self, s: Segment2) -> f64 {
        if self.intersect(s) {
            return 0.;
        }

        let distance_1 = self.distance_from_point(s.0);
        let distance_2 = self.distance_from_point(s.1);
        let distance_3 = s.distance_from_point(self.0);
        let distance_4 = s.distance_from_point(self.1);

        distance_1.min(distance_2).min(distance_3).min(distance_4)
    }

    /// 線分 s1 と線分 s2 の交点
    #[allow(dead_code)]
    fn cross_point(&self, s: Segment2) -> Point2 {
        let base = s.to_vector();
        let d1 = base.cross(self.0 - s.0).abs();
        let d2 = base.cross(self.1 - s.0).abs();
        let t = d1 / (d1 + d2);
        self.0 + (self.1 - self.0) * t
    }
}

/// 線分 s1 と線分 s2 の交点
#[allow(dead_code)]
fn cross_point(s1: Segment2, s2: Segment2) -> Point2 {
    s1.cross_point(s2)
}

impl Circle {
    /// 円と直線 の交差判定
    #[allow(dead_code)]
    fn intersect_line(&self, line: Segment2) -> bool {
        line.distance_between_line_and_point(self.center) <= self.radius
    }

    /// 円と直線の交点
    #[allow(dead_code)]
    fn cross_points_with_line(&self, line: Segment2) -> (Point2, Point2) {
        // 交差していること
        assert!(self.intersect_line(line));

        let pr = line.project(self.center);
        let e = line.to_vector() / line.to_vector().abs();
        let base = (self.radius * self.radius - (pr - self.center).norm()).sqrt();

        (pr + e * base, pr - e * base)
    }

    /// 円と円 の交差判定
    #[allow(dead_code)]
    fn intersect_circle(&self, rhs: Circle) -> bool {
        (self.center - rhs.center).abs() <= self.radius + rhs.radius
    }

    /// 円と円の交点
    #[allow(dead_code)]
    fn cross_points_with_circle(&self, rhs: Circle) -> (Point2, Point2) {
        assert!(self.intersect_circle(rhs));

        let d = (self.center - rhs.center).abs();
        let a = ((self.radius * self.radius + d * d - rhs.radius * rhs.radius)
            / (2. * self.radius * d))
            .acos()
            .to_radian();
        let t = (rhs.center - self.center).arg();

        (
            self.center + Point2::poloar(self.radius, t + a),
            self.center + Point2::poloar(self.radius, t - a),
        )
    }
}

#[allow(dead_code)]
enum PolygonPointContainment {
    In = 2,
    On = 1,
    Out = 3,
}

impl Polygon {
    /// 面積
    #[allow(dead_code)]
    fn area(&self) -> f64 {
        let mut sum = 0.;

        let n = self.0.len();

        for i in 0..n {
            let next_1 = (i + 1) % n;

            sum += self.0[i].cross(self.0[next_1]);
        }

        sum /= 2.;

        sum
    }

    /// 凸性判定
    #[allow(dead_code)]
    fn is_convex(&self) -> bool {
        let n = self.0.len();

        for i in 0..n {
            let next_1 = (i + 1) % n;
            let next_2 = (i + 2) % n;

            let a = self.0[next_1] - self.0[i];
            let b = self.0[next_2] - self.0[next_1];

            if a.cross(b) < 0. {
                return false;
            }
        }

        true
    }

    /// 点の内包
    #[allow(dead_code)]
    fn contains(&self, point: Point2) -> PolygonPointContainment {
        let n = self.0.len();
        let mut x = false;

        for i in 0..n {
            let mut a = self.0[i] - point;
            let mut b = self.0[(i + 1) % n] - point;

            if a.cross(b).abs() < EPS && a.dot(b) < EPS {
                return PolygonPointContainment::On;
            }

            if a.y > b.y {
                std::mem::swap(&mut a, &mut b);
            }

            if a.y < EPS && EPS < b.y && a.cross(b) > EPS {
                x = !x;
            }
        }

        if x {
            PolygonPointContainment::In
        } else {
            PolygonPointContainment::Out
        }
    }

    /// 凸包
    ///
    /// アンドリューのアルゴリズム
    #[allow(dead_code)]
    fn convex_hull(list: &Vec<Point2>) -> Polygon {
        let mut list = list.clone();

        if list.len() < 3 {
            return Polygon(list);
        }

        list.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .unwrap()
                .then(a.y.partial_cmp(&b.y).unwrap())
        });

        let mut upper = Vec::new();

        upper.push(list[0]);
        upper.push(list[1]);

        for i in 2..list.len() {
            while upper.len() >= 2
                && Point2::counter_clockwise(
                    upper[upper.len() - 2],
                    upper[upper.len() - 1],
                    list[i],
                    // 凸包の辺上の点を含めない場合
                    // ) != CcwPattern::Clockwise
                    // 凸包の辺上の点を含める場合
                ) == CcwPattern::CounterClockwise
            {
                upper.pop();
            }

            upper.push(list[i]);
        }

        let mut lower = Vec::new();

        lower.push(list[list.len() - 1]);
        lower.push(list[list.len() - 2]);

        for i in (0..list.len() - 2).rev() {
            while lower.len() >= 2
                && Point2::counter_clockwise(
                    lower[lower.len() - 2],
                    lower[lower.len() - 1],
                    list[i],
                    // 凸包の辺上の点を含めない場合
                    // ) != CcwPattern::Clockwise
                    // 凸包の辺上の点を含める場合
                ) == CcwPattern::CounterClockwise
            {
                lower.pop();
            }

            lower.push(list[i]);
        }

        lower.reverse();

        for i in (1..upper.len() - 1).rev() {
            lower.push(upper[i]);
        }

        Polygon(lower)
    }
}

///
/// 余弦定理
///
/// ```
/// assert_eq!(cos_formula(3., 4., 0.5 * std::f64::consts::PI,), 5);
/// ```
///
#[allow(dead_code)]
fn cos_formula(a: f64, b: f64, c_angle_radian: f64) -> f64 {
    (a * a + b * b - 2. * a * b * c_angle_radian.cos()).sqrt()
}
