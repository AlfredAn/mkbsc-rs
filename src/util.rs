use crate::IndexType;
use std::marker::PhantomData;
use fixedbitset::FixedBitSet;
use std::fmt;
use std::{ops::{Index, Range, Deref}, rc::Rc, iter::Map};
use array_init::array_init;
use typenum::*;
use itertools::*;
use std::{iter, cell::RefCell};

pub trait IntoCloneIterator: IntoIterator
where
    Self::IntoIter: Clone {}

impl<I> IntoCloneIterator for I
where
    I: IntoIterator,
    I::IntoIter: Clone {}

/*pub struct ItrFnOnce<T, F>(Option<F>, PhantomData<T>)
where
    F: FnOnce() -> T;

impl<T, F> ItrFnOnce<T, F>
where
    F: FnOnce() -> T
{
    pub fn new(x: F) -> Self {
        Self(Some(x), Default::default())
    }
}

impl<T, F> Iterator for ItrFnOnce<T, F>
where
    F: FnOnce() -> T
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(f) = self.0 {
            let result = f();
            self.0 = None;
            Some(result)
        } else {
            None
        }
    }
}*/

/*pub trait CopySet<T: Copy> {
    type Iter<'a>: Iterator<Item=T> where Self: 'a, T: 'a;

    fn contains(&self, x: T) -> bool;
    fn set_iter(&self) -> Self::Iter<'_>;
}

impl CopySet<usize> for FixedBitSet {
    type Iter<'a> = fixedbitset::Ones<'a>;

    fn contains(&self, x: usize) -> bool { self.contains(x) }
    fn set_iter(&self) -> Self::Iter<'_> { self.ones() }
}

pub struct FixedListMap<T, I: IndexType> {
    map: Vec<Option<T>>,
    list: Vec<I>
}

impl<T, I: IndexType> FixedListMap<T, I> {
    pub fn new(cap: usize) -> Self {
        Self {
            map: Vec::with_capacity(cap),
            list: Vec::with_capacity(cap)
        }
    }
}

impl<T, I: IndexType> CopySet<I> for FixedListMap<T, I> {
    type Iter<'a> = iter::Copied<std::slice::Iter<'a, I>> where T: 'a;

    fn contains(&self, x: I) -> bool {
        self.map[x.index()].is_some()
    }
    fn set_iter(&self) -> Self::Iter<'_> {
        self.list.iter().copied()
    }
}*/

pub type Itr<'a, T> = Box<dyn Iterator<Item=T> + 'a>;

pub trait CustomIterator: Iterator + Sized {
    /*fn introduce<T>(self, x: T) -> Introduce<Self, T> {
        Introduce(self, x)
    }*/
}

impl<I: Iterator> CustomIterator for I {}

/*pub struct Introduce<'a, I: Iterator, T>(I, RefCell<T>, PhantomData<&'a ()>);

impl<'a, I: Iterator, T: 'a> Iterator for Introduce<'a, I, T> {
    type Item = (I::Item, &'a RefCell<T>);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x, &self.1))
    }
}*/

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

/*pub fn iterator_product<I, const N: usize>(x: [I; N]) -> impl Iterator<Item=[I::Item; N]>
where
    I: IntoIterator,
    I::Item: Clone + Sized
{
    let vs = x.map(|itr| itr.into_iter().collect_vec());
    let range = array_init(|i| 0..(&vs[i]).len());
    index_product(vs, range)
}*/

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

/*pub fn index_power_vec<T, const N: usize>(x: [Vec<T>; N]) -> impl Iterator<Item=[T; N]>
where
    T: Clone
{
    let range = x.map(|v| 0..v.len());

}*/

pub fn index_power_generic<'a, T, const N: usize>(x: T, range: Range<usize>) -> impl Iterator<Item=[T::Output; N]> + 'a
where
    T: Index<usize> + 'a,
    T::Output: Clone + Sized
{
    //map_array(range_power(range), move |&i| x[i].clone())
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
