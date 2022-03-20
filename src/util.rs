use std::ops::{Index, Range};
use array_init::array_init;
use typenum::*;

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

pub trait TypeNumberTrait {
    type N: Unsigned;
}
pub struct TypeNumber<const N: usize> {}

macro_rules! impl_tn {
    ($n:expr, $t:ty) => {
        impl TypeNumberTrait for TypeNumber<{$n}> {
            type N = $t;
        }
    };
    ($n:expr, $t:ty, $($tail:tt)*) => {
        impl_tn!($n, $t);
        impl_tn!($n+1, $($tail)*);
    };
}

impl_tn!(0, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18,
    U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31, U32, U33, U34, U35, U36, U37,
    U38, U39, U40, U41, U42, U43, U44, U45, U46, U47, U48, U49, U50, U51, U52, U53, U54, U55, U56,
    U57, U58, U59, U60, U61, U62, U63, U64, U65, U66, U67, U68, U69, U70, U71, U72, U73, U74, U75,
    U76, U77, U78, U79, U80, U81, U82, U83, U84, U85, U86, U87, U88, U89, U90, U91, U92, U93, U94,
    U95, U96, U97, U98, U99, U100, U101, U102, U103, U104, U105, U106, U107, U108, U109, U110,
    U111, U112, U113, U114, U115, U116, U117, U118, U119, U120, U121, U122, U123, U124, U125, U126,
    U127);
