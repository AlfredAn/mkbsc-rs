use std::ops::{Index, Range};
use array_init::array_init;

pub fn index_power<T, const M: usize, const N: usize>(x: [T; M]) -> impl Iterator<Item=[T; N]>
where
    T: Clone
{
    index_power_generic(x, 0..M)
}

pub fn index_power_generic<T, const N: usize>(x: T, range: Range<usize>) -> impl Iterator<Item=[T::Output; N]>
where
    T: Index<usize>,
    T::Output: Clone + Sized
{
    map_array(range_power(range), move |&i| x[i].clone())
}

pub fn map_array<In, Out, I, F, const N: usize>(itr: I, mut f: F) -> impl Iterator<Item=[Out; N]>
where
    I: Iterator<Item=[In; N]>,
    F: FnMut(&In) -> Out
{
    itr.map(move |x| array_init(|i| f(&x[i])))
}

pub fn range_power<const N: usize>(range: Range<usize>) -> impl Iterator<Item=[usize; N]> {
    RangePower { index: [range.start; N], range: range, index_filter: |_| true }
}

pub fn range_power_filtered<IF, const N: usize>(range: Range<usize>, index_filter: IF) -> impl Iterator<Item=[usize; N]>
where
    IF: Fn(usize) -> bool
{
    RangePower { index: [range.start; N], range: range, index_filter: index_filter }
}

#[derive(Clone, Debug)]
pub struct RangePower<IF, const N: usize>
where
    IF: Fn(usize) -> bool
{
    range: Range<usize>,
    index: [usize; N],
    index_filter: IF
}

impl<IF, const N: usize> Iterator for RangePower<IF, N>
where
    IF: Fn(usize) -> bool
{
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if N == 0 || self.index[0] == self.range.end {
            return None;
        }

        let result = self.index;

        let mut has_next = false;
        for (j, i) in &mut self.index.iter_mut().enumerate() {
            if !(self.index_filter)(j) {
                continue;
            }
            *i += 1;
            if self.range.contains(i) {
                has_next = true;
                break;
            } else {
                *i = self.range.start;
            }
        }
        if !has_next {
            self.index[0] = self.range.end;
        }

        Some(result)
    }
}
