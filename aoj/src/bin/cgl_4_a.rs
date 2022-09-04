fn main() {
    let n = input_usize();

    let mut list = Vec::new();

    for _ in 0..n {
        let p = input_tuple();
        list.push(p);
    }

    let polygon = Polygon::convex_hull(&list);

    let mut start_i = 0;
    let mut start_point = polygon.0[0];

    for i in 0..polygon.0.len() {
        let p = polygon.0[i];

        if p.1 < start_point.1 || (p.1 == start_point.1 && p.0 < start_point.0) {
            start_point = p;
            start_i = i;
        }
    }

    println!("{}", polygon.0.len());

    for i in 0..polygon.0.len() {
        let p = polygon.0[(start_i + i) % polygon.0.len()];
        println!("{}", p);
    }
}

fn input_usize() -> usize {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}

fn input_tuple() -> Point2 {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let v1: f64 = iter.next().unwrap().parse().unwrap();
    let v2: f64 = iter.next().unwrap().parse().unwrap();

    Point2(v1, v2)
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
struct Point2(f64, f64);

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

impl std::ops::Div<f64> for Point2 {
    type Output = Point2;

    fn div(self, k: f64) -> Self {
        Point2(self.0 / k, self.1 / k)
    }
}

impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
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
        self.0 * self.0 + self.1 * self.1
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
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }

    /// ユークリッド距離
    /// 2点間の距離
    #[allow(dead_code)]
    fn euclidean_distance(&self, rhs: Point2) -> f64 {
        ((self.0 - rhs.0).powf(2.) + (self.1 - rhs.1).powf(2.)).sqrt()
    }

    /// 内積
    #[allow(dead_code)]
    fn dot(&self, rhs: Point2) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    /// 外積
    #[allow(dead_code)]
    fn cross(&self, rhs: Point2) -> f64 {
        self.0 * rhs.1 - self.1 * rhs.0
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
        Radian(self.1.atan2(self.0))
    }

    /// 極座標から変換
    #[allow(dead_code)]
    fn poloar(r: f64, theta: Radian) -> Point2 {
        Point2(theta.0.cos() * r, theta.0.sin() * r)
    }

    /// 任意点周りの回転移動（アフィン変換）
    /// https://imagingsolution.net/math/rotate-around-point/
    #[allow(dead_code)]
    fn rotate(&self, center: Point2, angle: Radian) -> Point2 {
        Point2(
            self.0 * angle.0.cos() - self.1 * angle.0.sin() + center.0 - center.0 * angle.0.cos()
                + center.1 * angle.0.sin(),
            self.0 * angle.0.sin() + self.1 * angle.0.cos() + center.1
                - center.0 * angle.0.sin()
                - center.1 * angle.0.cos(),
        )
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

            if a.1 > b.1 {
                std::mem::swap(&mut a, &mut b);
            }

            if a.1 < EPS && EPS < b.1 && a.cross(b) > EPS {
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
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
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
