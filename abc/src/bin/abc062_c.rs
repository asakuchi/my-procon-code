use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    if w % 3 == 0 || h % 3 == 0 {
        println!("0");
        return;
    }

    let mut result = h * w;

    // 横に3分割
    // 割り切れない場合
    // 横の幅の差は必ず1
    // ex. 8 => 3,3,2
    //     7 => 3,2,2
    // つまり 面積としての差は w
    result = result.min(w);

    // 縦も同じ考え方
    result = result.min(h);

    // 初めに横に分割してその後1つを縦に分割
    for i in 1..=h - 1 {
        let area_1 = i * w;
        let area_2 = (h - i) * (w / 2);
        let area_3 = if w % 2 == 0 {
            area_2
        } else {
            (h - 1) * (w / 2 + 1)
        };

        let min_area = area_1.min(area_2.min(area_3));
        let max_area = area_1.max(area_2.max(area_3));

        result = result.min(max_area - min_area);
    }

    // 初めに縦に分割その後1つを横に分割
    for i in 1..=w - 1 {
        let area_1 = h * i;
        let area_2 = (h / 2) * (w - i);
        let area_3 = if h % 2 == 0 {
            area_2
        } else {
            (h / 2 + 1) * (w - i)
        };

        let min_area = area_1.min(area_2.min(area_3));
        let max_area = area_1.max(area_2.max(area_3));

        result = result.min(max_area - min_area);
    }

    println!("{}", result);
}
