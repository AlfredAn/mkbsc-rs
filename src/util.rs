#[allow(dead_code)]

use fixedbitset::FixedBitSet;
use std::ops::Range;
use array_init::array_init;
use itertools::*;

pub fn fbs_intersection_check_empty(s: &mut FixedBitSet, s2: &FixedBitSet) -> bool {
    assert_eq!(s.len(), s2.len());

    let mut empty = true;
    for (b, b2) in izip!(s.as_mut_slice(), s2.as_slice()) {
        *b &= b2;
        empty &= *b == 0;
    }
    empty
}

pub fn fbs_filter(s: &mut FixedBitSet, mut f: impl FnMut(usize) -> bool) -> usize {
    let mut count = 0;

    for (i, block) in s.as_mut_slice().iter_mut().enumerate() {
        let i = i * 32;
        let mut temp = *block;

        while temp != 0 {
            // borrowed from FixedBitSet source code
            let t = temp & 0_u32.wrapping_sub(temp);
            let r = temp.trailing_zeros() as usize;
            temp ^= t;

            if f(i + r) {
                count += 1;
            } else {
                *block ^= t;
            }
        }
    }

    count
}

pub fn cartesian_product_generic<T, const N: usize>(
    x: impl Fn(usize, usize) -> T,
    len: [usize; N],
    mut f: impl FnMut([T; N])
) {
    if len.contains(&0) {
        return;
    }

    let mut i = [0; N];
    'outer: loop {
        f(array_init(|j|
            x(j, i[j])
        ));

        for j in 0..N {
            i[j] += 1;
            if i[j] < len[j] {
                continue 'outer;
            } else {
                i[j] = 0;
            }
        }
        break;
    }
}

pub fn cartesian_product_ints<T, const N: usize>(max: [usize; N], f: impl FnMut([usize; N])) {
    cartesian_product_generic(
        |_, i| i,
        max,
        f
    );
}

pub fn cartesian_product<T, const N: usize>(x: [&[T]; N], f: impl FnMut([&T; N])) {
    cartesian_product_generic(
        |j, i| &x[j][i],
        x.map(|y| y.len()),
        f
    );
}

pub fn range_product<const N: usize>(range: [Range<usize>; N]) -> impl Iterator<Item=[usize; N]> {
    RangeProduct { index: array_init(|i| range[i].start), range: move |i| range[i].clone(), index_filter: |_| true }
}

pub fn range_power<const N: usize>(range: Range<usize>) -> RangeProduct<impl Fn(usize) -> bool, impl Fn(usize) -> Range<usize>, N> {
    RangeProduct { index: [range.start; N], range: move |_| range.clone(), index_filter: |_| true }
}

pub fn range_power_filtered<IF, const N: usize>(range: Range<usize>, index_filter: IF) -> impl Iterator<Item=[usize; N]>
where
    IF: Fn(usize) -> bool
{
    RangeProduct { index: [range.start; N], range: move |_| range.clone(), index_filter: index_filter }
}

#[derive(Clone, Debug)]
pub struct RangeProduct<IF, R, const N: usize>
where
    IF: Fn(usize) -> bool,
    R: Fn(usize) -> Range<usize>
{
    range: R,
    index: [usize; N],
    index_filter: IF
}

impl<IF, R, const N: usize> Iterator for RangeProduct<IF, R, N>
where
    IF: Fn(usize) -> bool,
    R: Fn(usize) -> Range<usize>
{
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 || (0..N).any(|i| self.index[i] >= (self.range)(i).end) {
            return None;
        }

        let result = self.index;

        let mut has_next = false;
        for (j, i) in &mut self.index.iter_mut().enumerate() {
            if !(self.index_filter)(j) {
                continue;
            }
            *i += 1;
            if (self.range)(j).contains(i) {
                has_next = true;
                break;
            } else {
                *i = (self.range)(j).start;
            }
        }
        if !has_next {
            self.index[0] = (self.range)(0).end;
        }

        Some(result)
    }
}
