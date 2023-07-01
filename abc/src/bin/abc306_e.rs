/**
 * マルチセット
 */
use std::collections::btree_set::Iter;
use std::collections::BTreeSet;
use std::ops::Bound::*;
use std::ops::RangeBounds;

use proconio::{input, marker::Usize1};

struct MultiSet {
    uniq_index: usize,
    set: BTreeSet<(usize, usize)>,
}

impl<'a> MultiSet {
    fn new() -> MultiSet {
        MultiSet {
            uniq_index: 0,
            set: BTreeSet::new(),
        }
    }

    fn iter(&'a self) -> MultiSetIter {
        MultiSetIter {
            iter: self.set.iter(),
        }
    }

    fn range<R>(&'a self, range: R) -> impl Iterator<Item = usize> + 'a
    where
        // K: Ord + ?Sized,
        // T: Borrow<K> + Ord,
        R: RangeBounds<usize>,
    {
        let range_from = |start| self.set.range((start, 0)..);
        let range_to = |end| self.set.range(..(end, std::usize::MAX));
        let range_both = |start, end| self.set.range((start, 0)..(end, std::usize::MAX));

        let iter = match (range.start_bound(), range.end_bound()) {
            // Included と Excluded は区別する必要なし
            (Unbounded, Unbounded) => self.set.range(..),
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

    fn insert(&mut self, value: usize) -> bool {
        self.uniq_index += 1;
        self.set.insert((value, self.uniq_index))
    }

    fn inner_find(&'a self, value: usize) -> Option<&'a (usize, usize)> {
        self.set.range((value, 0)..=(value, std::usize::MAX)).next()
    }

    fn remove(&mut self, value: usize) -> bool {
        if let Some(&find_data) = self.inner_find(value) {
            self.set.remove(&find_data)
        } else {
            false
        }
    }

    fn get(&self, value: usize) -> Option<usize> {
        if let Some(&(inner_value, _)) = self.inner_find(value) {
            Some(inner_value)
        } else {
            None
        }
    }

    fn contains(&self, value: usize) -> bool {
        if let Some(_) = self.inner_find(value) {
            true
        } else {
            false
        }
    }
}

pub struct MultiSetIter<'a> {
    iter: Iter<'a, (usize, usize)>,
}

impl<'a> Iterator for MultiSetIter<'a> {
    type Item = &'a usize;
    // type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.iter.next()

        if let Some((value, _index)) = self.iter.next() {
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    // fn last(mut self) -> Option<&'a T> {
    fn last(mut self) -> Option<&'a usize> {
        // self.next_back()

        if let Some((value, _index)) = self.iter.last() {
            Some(value)
        } else {
            None
        }
    }

    // fn min(mut self) -> Option<&'a T> {
    fn min(mut self) -> Option<&'a usize> {
        // self.next()

        if let Some((value, _index)) = self.iter.next() {
            Some(value)
        } else {
            None
        }
    }

    // fn max(mut self) -> Option<&'a T> {
    fn max(mut self) -> Option<&'a usize> {
        // self.next_back()

        if let Some((value, _index)) = self.iter.last() {
            Some(value)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for MultiSetIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((value, _index)) = self.iter.next_back() {
            Some(value)
        } else {
            None
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

    let mut upper_value = MultiSet::new();
    let mut lower_value = MultiSet::new();

    for _ in 0..k {
        // 暫定的に先頭k項を上位にする
        upper_value.insert(0);
    }

    for _ in k..n {
        lower_value.insert(0);
    }

    for q_i in 0..q {
        let (x, y) = x_y[q_i];

        let old_value = a[x];

        a[x] = y;

        // f(A) を出す

        // old_valueが上位にいるなら更新
        if upper_value.contains(old_value) {
            upper_value.remove(old_value);

            // 暫定的に入れる
            lower_value.insert(y);

            if let Some(&next_value) = lower_value.iter().last() {
                upper_value.insert(next_value);
                lower_value.remove(next_value);

                top_k -= old_value;
                top_k += next_value;
            }
        } else {
            // 下位にいるはずなので更新
            lower_value.remove(old_value);

            // 新しい値は上位にいるべきか
            let option = upper_value.range(..y).next();

            if let Some(less_value) = option {
                // 下位 -> 上位

                upper_value.insert(y);

                // 繰り下がり
                upper_value.remove(less_value);
                lower_value.insert(less_value);

                top_k -= less_value;
                top_k += y;
            } else {
                // 下位 -> 下位

                // 下位のまま
                lower_value.insert(y);

                // このパターンはtop_kの更新なし
            }
        }

        println!("{}", top_k);
    }
}
