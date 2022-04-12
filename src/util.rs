#[allow(dead_code)]

use fixedbitset::FixedBitSet;
use std::{ops::{Index, Range}, rc::Rc};
use array_init::array_init;
use std::{cell::RefCell};

/// panics if any of the slices are empty
pub fn cartesian_product<T, const N: usize>(x: [&[T]; N], mut f: impl FnMut([&T; N])) {
    let mut i = [0; N];
    'outer: loop {
        f(array_init(|j|
            &x[j][i[j]]
        ));

        for j in 0..N {
            i[j] += 1;
            if i[j] < x[j].len() {
                continue 'outer;
            } else {
                i[j] = 0;
            }
        }
        break;
    }
}

pub type Itr<'a, T> = Box<dyn Iterator<Item=T> + 'a>;

pub fn unique_fbs<T: Into<usize> + Copy>(itr: impl IntoIterator<Item=T>, cap: usize) -> impl Iterator<Item=T> {
    UniqueFBS {
        set: FixedBitSet::with_capacity(cap),
        itr: itr.into_iter()
    }
}

pub struct UniqueFBS<I> {
    set: FixedBitSet,
    itr: I
}

impl<I> Iterator for UniqueFBS<I>
where
    I: Iterator,
    I::Item: Into<usize> + Copy
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        if let Some(next) = self.itr.next() {
            if !self.set.put(next.into()) {
                return Some(next);
            }
        }
        None
    }
}

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

macro_rules! iterator_product {
    ($x:expr) => {
        {
            let x = $x;
            let vs = x.map(|itr| itr.into_iter().collect_vec());
            let range = array_init::array_init(|i| 0..vs[i].len());
            crate::util::index_product(vs, range)
        }
    };
}

pub(crate) use iterator_product;

pub fn index_product<T, const N: usize>(x: [Vec<T>; N], range: [Range<usize>; N]) -> impl Iterator<Item=[T; N]>
where
    T: Clone
{
    let r = RangeProduct { index: array_init::<_, _, N>(|i| range[i].start), range: move |i| range[i].clone(), index_filter: |_| true };
    let r = r
        .map(move |is|
            array_init(|j| {
                let i = is[j];
                x[j][i].clone()
            })
        );

    r
}

pub fn index_power<'a, T, const M: usize, const N: usize>(x: [T; M]) -> impl Iterator<Item=[T; N]> + 'a
where
    T: Clone + 'a
{
    let r = range_power::<N>(0..M).map(move |is| array_init(|i| x[is[i]].clone()));
    r
}

pub fn index_power_generic<'a, T, const N: usize>(x: T, range: Range<usize>) -> impl Iterator<Item=[T::Output; N]> + 'a
where
    T: Index<usize> + 'a,
    T::Output: Clone + Sized
{
    let r = range_power::<N>(range).map(move |is| array_init(|i| x[is[i]].clone()));
    r
}

pub fn map_array<In, Out, I, F, const N: usize>(itr: I, mut f: F) -> impl Iterator<Item=[Out; N]>
where
    I: Iterator<Item=[In; N]>,
    F: FnMut(&In) -> Out + Clone
{
    itr.map(move |x| array_init(|i| f(&x[i])))
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
