use proconio::input;
use superslice::Ext;

#[proconio::fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        q: usize,
        mut s: [usize; a],
        mut t: [usize; b],
        x: [usize; q],
    }

    // 番兵
    s.push(1_000_000_000_000);
    t.push(1_000_000_000_000);

    for x_i in x {
        let s_i = s.lower_bound(&x_i);
        let t_i = t.lower_bound(&x_i);

        let s_r = s_i;
        let t_r = t_i;

        // 神社も寺も右側
        let r_r = s[s_r].max(t[t_r]) - x_i;
        let mut result = r_r;

        if let Some(s_l) = if s_i == 0 { None } else { Some(s_i - 1) } {
            if let Some(t_l) = if t_i == 0 { None } else { Some(t_i - 1) } {
                // 神社も寺も左側
                let l_l = x_i - s[s_l].min(t[t_l]);
                result = result.min(l_l);
            }

            // 神社は左、寺は右
            let l_r = (t[t_r] - s[s_l]) + (x_i - s[s_l]).min(t[t_r] - x_i);
            result = result.min(l_r);
        }

        if let Some(t_l) = if t_i == 0 { None } else { Some(t_i - 1) } {
            // 神社は右、寺は左
            let r_l = (s[s_r] - t[t_l]) + (x_i - t[t_l]).min(s[s_r] - x_i);
            result = result.min(r_l);
        }

        println!("{}", result);
    }
}
