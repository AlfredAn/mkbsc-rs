use std::cell::Cell;
use derive_more::*;
use itertools::structs::GroupBy;

#[allow(dead_code)]

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct ReverseOrd<T>(T);

impl<T: PartialOrd> PartialOrd for ReverseOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
            .map(|o| o.reverse())
    }
}

impl<T: Ord> Ord for ReverseOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

pub trait CustomArray<T, const N: usize>: IndexMut<usize, Output=T> + Into<[T; N]>
where
    for<'a> &'a Self: Into<&'a [T; N]>,
    for<'a> &'a mut Self: Into<&'a mut [T; N]>
{
    fn ref_array(&self) -> [&T; N] {
        array_init(|i| &self[i])
    }

    fn ref_array_mut(&mut self) -> [&mut T; N] {
        let mut result = ArrayVec::new();
        result.extend(self.into().iter_mut());
        result.into_inner().unwrap_or_else(|_| unreachable!())
    }
}

impl<T, const N: usize> CustomArray<T, N> for [T; N] {}

pub type SortAndGroupBy<T, K, F> = GroupBy<K, std::vec::IntoIter<T>, F>;

pub trait CustomIterator: Iterator + Sized {
    fn sort_and_group_by_key<K, F>(self, mut f: F)
    -> SortAndGroupBy<Self::Item, K, F>
    where
        Self::Item: Debug + Clone,
        K: Ord,
        F: FnMut(&Self::Item) -> K
    {
        self.sorted_by(|a, b| f(a).cmp(&f(b)))
            .group_by(f)
    }

    fn collect_array<const N: usize>(self) -> Option<[Self::Item; N]> {
        let mut result = ArrayVec::new();
        for x in self {
            if result.try_push(x).is_err() {
                return None;
            }
        }
        result.into_inner().ok()
    }
}

impl<T: Iterator + Sized> CustomIterator for T {}

struct OpaqueDisplay<F: Fn(&mut fmt::Formatter) -> fmt::Result>(F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> Display for OpaqueDisplay<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> Debug for OpaqueDisplay<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

pub fn display(func: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl Display + Debug {
    OpaqueDisplay(func)
}

pub fn display_once(func: impl FnOnce(&mut fmt::Formatter) -> fmt::Result) -> impl Display + Debug {
    let cell = Cell::new(Some(func));
    OpaqueDisplay(move |f| (cell.take().unwrap())(f))
}

pub fn print(func: impl FnOnce(&mut fmt::Formatter) -> fmt::Result) {
    println!("{}", display_once(func));
}

#[allow(unused_macros)]
macro_rules! include_game {
    ($path:expr, $n:expr) => {{
        let s = include_str!($path);
        let parsed = parser::parse(s).unwrap();
        let g: io_game::IOGame<$n> = io_game::IOGameEnum::new(parsed)
            .unwrap()
            .try_into()
            .unwrap();
        g
    }};
}
pub(crate) use include_game;

#[allow(unused_macros)]
macro_rules! for_each_tuple {
    (($head:expr $(,$rest:expr)*$(,)?), $f:expr) => {
        ($f)($head);
        for_each_tuple!(($($rest),*), $f);
    };
    ((), $f:expr) => ();
}
pub(crate) use for_each_tuple;

pub fn find_group<T>(slice: &[T], mut cmp: impl FnMut(&T) -> std::cmp::Ordering) -> &[T] {
    let (mut lo, mut hi) = (0, slice.len());

    // println!("{:?}", slice.iter().map(|x| cmp(x)).collect_vec());

    let mid = loop {
        // println!("{:?}", (lo, hi));

        if lo == hi {
            return &[];
        }

        let mid = lo + (hi - lo)/2;
        match cmp(&slice[mid]) {
            Ordering::Less => (lo, hi) = (lo, mid),
            Ordering::Equal => break mid,
            Ordering::Greater => (lo, hi) = (mid+1, hi)
        }
    };

    // println!("{:?}", mid);
 
    let start = find_first(&slice[lo..mid+1], |x| cmp(x).is_le()) + lo;
    // println!("start = {:?}", start);
    let end = find_first(&slice[mid..hi], |x| cmp(x).is_lt()) + mid;
    // println!("end = {:?}", end);

    assert!(cmp(&slice[start]).is_eq());
    assert!(start == 0 || cmp(&slice[start-1]).is_gt());

    assert!(end == slice.len() || cmp(&slice[end]).is_lt());
    assert!(cmp(&slice[end-1]).is_eq());
    
    &slice[start..end]
}

/// Finds the index of the first element that satisfies the predicate.
/// 
/// Returns slice.len() if all elements are false.
/// 
/// Assumes that there exists an index i such that the predicate
/// returns false for all elements with index less than i
/// and true for all elements with index greater than or equal to i.
/// 
pub fn find_first<T>(slice: &[T], mut pred: impl FnMut(&T) -> bool) -> usize {
    let (mut lo, mut hi) = (0, slice.len());

    if lo == hi {
        return 0;
    }

    // println!("{:?}", slice.iter().map(|x| pred(x)).collect_vec());

    loop {
        // println!("{:?}", (lo, hi));

        assert!(lo <= hi);
        assert!(lo == 0 || !pred(&slice[lo-1]));
        assert!(hi == slice.len() || pred(&slice[hi]));

        match hi - lo {
            0 => {
                return hi;
            },
            1 => if pred(&slice[lo]) {
                return lo;
            } else {
                return hi;
            },
            _ => {
                let mid = lo + (hi - lo)/2;
                if pred(&slice[mid]) {
                    (lo, hi) = (lo, mid);
                } else {
                    (lo, hi) = (mid+1, hi);
                }
            }
        }
    }
}

pub struct PtrEqRc<T: ?Sized>(pub Rc<T>);

impl<T: ?Sized> Deref for PtrEqRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> Clone for PtrEqRc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Ord for PtrEqRc<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Rc::as_ptr(&self.0).cmp(&Rc::as_ptr(&other.0))
    }
}

impl<T: ?Sized> PartialOrd for PtrEqRc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<T: Debug + ?Sized> Debug for PtrEqRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PtrEqRc({:?})", &*self.0)
    }
}

impl<T: Display + ?Sized> Display for PtrEqRc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &*self.0)
    }
}

impl<T: ?Sized> PartialEq for PtrEqRc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Eq for PtrEqRc<T> {}

impl<T: ?Sized> Hash for PtrEqRc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

pub struct SequenceFormat<'a> {
    pub start: &'a str,
    pub end: &'a str,
    pub sep: &'a str
}

pub const LIST: SequenceFormat = SequenceFormat {
    start: "[",
    end: "]",
    sep: ", ",
};

pub const SET: SequenceFormat = SequenceFormat {
    start: "{",
    end: "}",
    sep: ", ",
};

pub fn format_sequence<T>(
    f: &mut fmt::Formatter,
    fmt: SequenceFormat,
    iter: impl IntoIterator<Item=T>,
    mut x: impl FnMut(&mut fmt::Formatter, T) -> fmt::Result
) -> fmt::Result {
    write!(f, "{}", fmt.start)?;

    let mut first = true;
    for a in iter.into_iter() {
        if !first {
            write!(f, "{}", fmt.sep)?;
        }
        first = false;
        
        x(f, a)?;
    }

    write!(f, "{}", fmt.end)
}

pub fn format_sep<T>(
    f: &mut fmt::Formatter,
    sep: &str,
    iter: impl IntoIterator<Item=T>,
    x: impl FnMut(&mut fmt::Formatter, T) -> fmt::Result
) -> fmt::Result {
    format_sequence(f, SequenceFormat { start: "", end: "", sep }, iter, x)
}

pub fn format_list<T>(
    f: &mut fmt::Formatter,
    iter: impl IntoIterator<Item=T>,
    x: impl FnMut(&mut fmt::Formatter, T) -> fmt::Result
) -> fmt::Result {
    format_sequence(f, LIST, iter, x)
}

pub fn format_set<T>(
    f: &mut fmt::Formatter,
    iter: impl IntoIterator<Item=T>,
    x: impl FnMut(&mut fmt::Formatter, T) -> fmt::Result
) -> fmt::Result {
    format_sequence(f, SET, iter, x)
}

pub fn fold_product<T, A, const N: usize>(
    init: A,
    len: [usize; N],
    mut fold: impl FnMut(usize, usize, &A) -> A,
    mut end: impl FnMut(A)
) {
    if len.contains(&0) {
        return;
    }

    let mut stack = ArrayVec::<_, N>::new();
    
    let mut i = [0; N];
    'outer: loop {
        while !stack.is_full() {
            let j = stack.len();
            stack.push(
                fold(
                    j,
                    i[j],
                    stack.last().unwrap_or(&init)
                )
            );
        }
        end(stack.pop().unwrap());

        for j in 0..N {
            i[j] += 1;
            if i[j] < len[j] {
                continue 'outer;
            } else {
                i[j] = 0;
                stack.pop();
            }
        }
        break;
    }
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

pub fn cartesian_product_ints<const N: usize>(max: [usize; N], f: impl FnMut([usize; N])) {
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
