use std::{ops::{Index, Range, Deref}, rc::Rc};
use array_init::array_init;
use typenum::*;
use itertools::Itertools;
use std::{iter, cell::RefCell};

pub type Itr<'a, T> = Box<dyn Iterator<Item=T> + 'a>;

pub fn into_cloneable<I>(itr: I) -> impl Iterator<Item=I::Item> + Clone
where
    I: Iterator,
    I::Item: Clone
{
    Cloneable::new(itr)
}

#[derive(Debug)]
struct Cloneable<I: Iterator> {
    data: Rc<RefCell<(I, Vec<I::Item>, bool)>>,
    i: usize
}

impl<I> Cloneable<I>
where
    I: Iterator,
    I::Item: Clone
{
    fn new(itr: I) -> Self {
        Self {
            data: Rc::new(RefCell::new((itr, Vec::new(), false))),
            i: 0
        }
    }
}

impl<I> Clone for Cloneable<I>
where
    I: Iterator,
    I::Item: Clone
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            i: self.i
        }
    }
}

impl<I> Iterator for Cloneable<I>
where
    I: Iterator,
    I::Item: Clone
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let mut c = self.data.borrow_mut();
        let i = self.i;

        if i < c.1.len() {
            self.i += 1;
            Some(c.1[i].clone())
        } else if !c.2 {
            self.i += 1;
            if let Some(x) = c.0.next() {
                c.1.push(x.clone());
                Some(x)
            } else {
                c.2 = true;
                None
            }
        } else {
            None
        }
    }
}

/*pub fn iter_clone<T, I>(i: &I) -> impl Iterator<Item=T>
where
    T: Clone,
    I: Iterator<Item=T>
{
    
}*/

pub trait Captures<'a>: 'a {}
impl<'a, T> Captures<'a> for T where T: 'a {}

pub trait Reference<'a>: Deref + 'a {}
impl<'a, T> Reference<'a> for T where T: Deref + 'a, T::Target: 'a {}

pub fn iterator_product<I, const N: usize>(x: [I; N]) -> impl Iterator<Item=[I::Item; N]>
where
    I: IntoIterator,
    I::Item: Clone + Sized
{
    let vs = x.map(|itr| itr.into_iter().collect_vec());
    let range = array_init(|i| 0..(&vs[i]).len());
    index_product(vs, range)
}

pub fn index_product<I, const N: usize>(x: [I; N], range: [Range<usize>; N]) -> impl Iterator<Item=[I::Output; N]> + Clone
where
    I: Index<usize> + Clone,
    I::Output: Clone + Sized
{
    let r = range_product(range)
        .map(move |is|
            array_init(|j| {
                let i = is[j];
                x[j][i].clone()
            })
        );

    r
}

pub fn index_power<T, const M: usize, const N: usize>(x: [T; M]) -> impl Iterator<Item=[T; N]> + Clone
where
    T: Clone
{
    index_power_generic(x, 0..M)
}

pub fn index_power_generic<T, const N: usize>(x: T, range: Range<usize>) -> impl Iterator<Item=[T::Output; N]> + Clone
where
    T: Index<usize> + Clone,
    T::Output: Clone + Sized
{
    map_array(range_power(range), move |&i| x[i].clone())
}

pub fn map_array<In, Out, I, F, const N: usize>(itr: I, mut f: F) -> impl Iterator<Item=[Out; N]> + Clone
where
    I: Iterator<Item=[In; N]> + Clone,
    F: FnMut(&In) -> Out + Clone
{
    itr.map(move |x| array_init(|i| f(&x[i])))
}

pub fn range_product<const N: usize>(range: [Range<usize>; N]) -> impl Iterator<Item=[usize; N]> + Clone {
    RangeProduct { index: array_init(|i| range[i].start), range: move |i| range[i].clone(), index_filter: |_| true }
}

pub fn range_power<const N: usize>(range: Range<usize>) -> impl Iterator<Item=[usize; N]> + Clone {
    RangeProduct { index: [range.start; N], range: move |_| range.clone(), index_filter: |_| true }
}

pub fn range_power_filtered<IF, const N: usize>(range: Range<usize>, index_filter: IF) -> impl Iterator<Item=[usize; N]> + Clone
where
    IF: Fn(usize) -> bool + Clone
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
        if N == 0 || self.index[0] == (self.range)(0).end {
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
