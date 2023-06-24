use std::collections::BTreeSet;
use std::ops::Bound::*;
use std::ops::RangeBounds;

use proconio::{input, marker::Usize1};

struct MultiSet {
    uniq_index: usize,
}

impl<'a> MultiSet {
    fn new() -> MultiSet {
        MultiSet { uniq_index: 0 }
    }

    fn range<R>(
        &self,
        set: &'a BTreeSet<(usize, usize)>,
        range: R,
    ) -> impl Iterator<Item = usize> + 'a
    where
        // K: Ord + ?Sized,
        // T: Borrow<K> + Ord,
        R: RangeBounds<usize>,
    {
        let range_from = |start| set.range((start, 0)..);
        let range_to = |end| set.range(..(end, std::usize::MAX));
        let range_both = |start, end| set.range((start, 0)..(end, std::usize::MAX));

        let iter = match (range.start_bound(), range.end_bound()) {
            // Included と Excluded は区別する必要なし
            (Unbounded, Unbounded) => set.range(..),
            (Included(&start_value), Unbounded) => range_from(start_value),
            (Excluded(&start_value), Unbounded) => range_from(start_value),
            (Unbounded, Included(&end_value)) => range_to(end_value),
            (Unbounded, Excluded(&end_value)) => range_to(end_value),
            (Included(&start_value), Included(&end_value)) => range_both(start_value, end_value),
            (Included(&start_value), Excluded(&end_value)) => range_both(start_value, end_value),
            (Excluded(&start_value), Included(&end_value)) => range_both(start_value, end_value),
            (Excluded(&start_value), Excluded(&end_value)) => range_both(start_value, end_value),
        };

        iter.map(|&(x, _)| x)
        // iter
    }

    fn insert(&mut self, set: &mut BTreeSet<(usize, usize)>, value: usize) -> bool {
        self.uniq_index += 1;
        set.insert((value, self.uniq_index))
    }

    fn inner_find(
        &self,
        set: &'a BTreeSet<(usize, usize)>,
        value: usize,
    ) -> Option<&'a (usize, usize)> {
        set.range((value, 0)..=(value, std::usize::MAX)).next()
    }

    fn remove(&mut self, set: &mut BTreeSet<(usize, usize)>, value: usize) -> bool {
        if let Some(&find_data) = self.inner_find(set, value) {
            set.remove(&find_data)
        } else {
            false
        }
    }

    fn get(&self, set: &BTreeSet<(usize, usize)>, value: usize) -> Option<usize> {
        if let Some(&(inner_value, _)) = self.inner_find(set, value) {
            Some(inner_value)
        } else {
            None
        }
    }

    fn contains(&self, set: &BTreeSet<(usize, usize)>, value: usize) -> bool {
        if let Some(_) = self.inner_find(set, value) {
            true
        } else {
            false
        }
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        x_y: [(Usize1, usize); q],
    }

    let mut a = vec![0; n];

    let mut top_k = 0_usize;

    let mut upper_value = BTreeSet::new();
    let mut lower_value = BTreeSet::new();

    let mut multi = MultiSet::new();

    for _ in 0..k {
        // 暫定的に先頭k項を上位にする
        multi.insert(&mut upper_value, 0);
    }

    for _ in k..n {
        multi.insert(&mut lower_value, 0);
    }

    for q_i in 0..q {
        let (x, y) = x_y[q_i];

        let old_value = a[x];

        a[x] = y;

        // f(A) を出す

        // old_valueが上位にいるなら更新
        if multi.contains(&upper_value, old_value) {
            multi.remove(&mut upper_value, old_value);

            // 暫定的に入れる
            multi.insert(&mut lower_value, y);

            if let Some(&(next_value, _)) = lower_value.iter().last() {
                multi.insert(&mut upper_value, next_value);
                multi.remove(&mut lower_value, next_value);

                top_k -= old_value;
                top_k += next_value;
            }
        } else {
            // 下位にいるはずなので更新
            multi.remove(&mut lower_value, old_value);

            // 新しい値は上位にいるべきか
            let option = multi.range(&upper_value, ..y).next();

            if let Some(less_value) = option {
                // 下位 -> 上位

                multi.insert(&mut upper_value, y);

                // 繰り下がり
                multi.remove(&mut upper_value, less_value);
                multi.insert(&mut lower_value, less_value);

                top_k -= less_value;
                top_k += y;
            } else {
                // 下位 -> 下位

                // 下位のまま
                multi.insert(&mut lower_value, y);

                // このパターンはtop_kの更新なし
            }
        }

        println!("{}", top_k);
    }
}
