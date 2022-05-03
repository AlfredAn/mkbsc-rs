use derive_more::*;
use itertools::izip;

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Add, Sub, AddAssign, SubAssign)]
pub struct Pos {
    x: i8,
    y: i8
}

macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos { x: $x, y: $y }
    };
}

const MOVE: [Pos; 5] = [
    pos!( 0,  0),
    pos!( 1,  0),
    pos!(-1,  0),
    pos!( 0,  1),
    pos!( 0, -1)
];

const VIS: [Pos; 5] = MOVE;

#[derive(Debug, Clone)]
pub struct GridPursuitGame<const X: i8, const Y: i8, const N: usize> {
    l0: Loc<X, Y, N>
}

#[derive(Debug, Copy, Clone, Eq, new)]
pub struct Loc<const X: i8, const Y: i8, const N: usize> {
    pu: [Pos; N],
    ev: Pos
}

impl<const X: i8, const Y: i8, const N: usize> Add for Loc<X, Y, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            array_init(|i| self.pu[i] + rhs.pu[i]),
            self.ev + rhs.ev
        )
    }
}

impl<const X: i8, const Y: i8, const N: usize> Sub for Loc<X, Y, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            array_init(|i| self.pu[i] - rhs.pu[i]),
            self.ev - rhs.ev
        )
    }
}

impl<const X: i8, const Y: i8, const N: usize> AddAssign for Loc<X, Y, N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const X: i8, const Y: i8, const N: usize> SubAssign for Loc<X, Y, N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

const SYM: [(u8, bool); 8] = [
    (0, false), (1, false), (2, false), (3, false),
    (0, true), (1, true), (2, true), (3, true)
];

fn sym<const X: i8, const Y: i8>(mut p: Pos, (rot, flip): (u8, bool)) -> Pos {
    if flip {
        p = pos!(X-1-p.x, p.y);
    }

    match rot & 3 {
        0 => p,
        1 => pos!(p.y, X-1-p.x),
        2 => pos!(X-1-p.x, Y-1-p.y),
        3 => pos!(Y-1-p.y, p.x),
        _ => unreachable!()
    }
}

impl<const X: i8, const Y: i8, const N: usize> Loc<X, Y, N> {
    fn is_winning(self) -> bool {
        self.pu.contains(&self.ev)
    }

    fn sym(self, s: (u8, bool)) -> Self {
        Self {
            pu: array_init::array_init(|a| sym::<X, Y>(self.pu[a], s)),
            ev: sym::<X, Y>(self.ev, s)
        }
    }

    fn shallow_eq(self, other: Self) -> bool {
        self.pu == other.pu && self.ev == other.ev
    }

    fn isomorphic(self, other: Self) -> bool {
        SYM.iter().any(|&s| self.sym(s).shallow_eq(other))
    }

    fn ord_value(self) -> u64 {
        let (mut result, mut mul) = (0, 1);

        let mut add = |val, max| { result += (val as u64)*mul; mul *= max as u64 };

        add(self.ev.x, X);
        add(self.ev.y, Y);

        for a in self.pu {
            add(a.x, X);
            add(a.y, Y);
        }

        result
    }

    fn canonical(self) -> (Self, u64) {
        SYM[1..].iter().map(|&s| self.sym(s)).fold((self, self.ord_value()), |(a, ax), b| {
            let bx = b.ord_value();
            if ax < bx { (a, ax) } else { (b, bx) }
        })
    }

    fn fmt_compact(&self) -> String {
        if N != 2 {
            todo!();
        } else if self.is_winning() {
            "W".into()
        } else {
            format!("{}{}{}{}{}{}", self.pu[0].x, self.pu[0].y, self.pu[1].x, self.pu[1].y, self.ev.x, self.ev.y)
        }
    }
}

impl<const X: i8, const Y: i8, const N: usize> PartialEq for Loc<X, Y, N> {
    fn eq(&self, other: &Self) -> bool {
          (self.is_winning() && other.is_winning())
        || self.isomorphic(*other)
    }
}

impl<const X: i8, const Y: i8, const N: usize> Hash for Loc<X, Y, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_winning() {
            'w'.hash(state)
        } else {
            self.canonical().1.hash(state)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, new)]
pub struct Edge<const X: i8, const Y: i8, const N: usize> {
    from: Loc<X, Y, N>,
    to: Loc<X, Y, N>,
    act: Action<N>
}

type Action<const N: usize> = [Pos; N];

const fn act_zero<const N: usize>() -> Action<N> {
    [pos!(0, 0); N]
}

impl<const X: i8, const Y: i8, const N: usize> AbstractGame<N> for GridPursuitGame<X, Y, N> {
    type Loc = Loc<X, Y, N>;
    type Obs = Obs;

    fn l0(&self) -> Self::Loc {
        self.l0
    }

    fn is_winning(&self, l: &Self::Loc) -> bool {
        l.is_winning()
    }

    fn n_actions(&self) -> [usize; N] {
        [MOVE.len(); N]
    }

    fn obs(&self, l: &Self::Loc) -> [Self::Obs; N] {
        array_init(|i| {
            let p = l.pu[i];
            Obs(p, array_init(|j| {
                let p_vis = p + VIS[j];
                let has_evader = l.ev == p_vis;
                if let Some((p2, _)) = l.pu.iter().find_position(|&&p2| p2 == p_vis) {
                    SquareObs::new(Some(p2 as u8), has_evader)
                } else {
                    SquareObs::new(None, has_evader)
                }
            }))
        })
    }

    fn succ(
        &self,
        l: &Self::Loc,
        mut f: impl FnMut([Act; N], Self::Loc)
    ) {
        for e in Edges::new(*l) {
            let act = array_init(|i|
                MOVE.iter().find_position(|&&p| p == l.pu[i]).unwrap().0
            );

            f(act, e.to);
        }
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, l: &Self::Loc) -> fmt::Result {
        write!(f, "{}", l.fmt_compact())
    }

    /*fn post(&self, n: &Self::Loc, a: Action<N>) -> Itr<Self::Loc> {
        //todo: optimize
        Box::new(Edges::new(*n).filter(move |e| e.act == a).map(|e| e.to))
    }

    fn actions(&self) -> Itr<[Self::Act; N]> {
        Box::new(index_power(MOVE))
    }


    fn observe(&self, l: &Self::Loc) -> [Obs; N] {
        array_init(|i| {
            let p = l.pu[i];
            Obs(p, array_init(|j| {
                let p_vis = p + VIS[j];
                let has_evader = l.ev == p_vis;
                if let Some((p2, _)) = l.pu.iter().find_position(|&&p2| p2 == p_vis) {
                    SquareObs::new(Some(p2 as u8), has_evader)
                } else {
                    SquareObs::new(None, has_evader)
                }
            }))
        })
    }

    type Agt = usize;

    fn actions_i(&self, _: Self::Agt) -> Itr<Self::Act> {
        Box::new(MOVE.iter().copied())
    }

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        if N == 2 {
            Some(l.fmt_compact())
        } else {
            todo!();
        }
    }

    derive_dgame!(N);*/
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SquareObs {
    pursuer: Option<u8>,
    evader: bool
}

impl SquareObs {
    pub fn new(pursuer: Option<u8>, evader: bool) -> Self { Self { pursuer, evader } }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Obs(Pos, [SquareObs; VIS.len()]);

impl<const X: i8, const Y: i8> Default for GridPursuitGame<X, Y, 2> {
    fn default() -> Self {
        Self { l0: Loc { pu: [pos!(X-1, 0), pos!(0, Y-1)], ev: pos!(0, 0) } }
    }
}

#[derive(Clone, Debug)]
pub struct Edges<const X: i8, const Y: i8, const N: usize> {
    l: Loc<X, Y, N>,
    finished: bool,
    i: [usize; N],
    j: usize,
    overlaps: ArrayVec<ArrayVec<usize, N>, N>,
    l2: Loc<X, Y, N>,
    act: Action<N>,
    k: [usize; N],
    k_fixed: [bool; N]
}

fn in_bounds<const X: i8, const Y: i8, const N: usize>(l: Loc<X, Y, N>) -> bool {
    l.pu.into_iter().chain(iter::once(l.ev)).all(|p| (0..X).contains(&p.x) && (0..Y).contains(&p.y))
}

fn overlaps<const X: i8, const Y: i8, const N: usize>(l: Loc<X, Y, N>) -> bool {
    for (i, ai) in l.pu.iter().enumerate() {
        for aj in l.pu.iter().skip(i+1) {
            if ai == aj {
                return true;
            }
        }
    }
    false
}

impl<const X: i8, const Y: i8, const N: usize> Edges<X, Y, N> {
    fn new(l: Loc<X, Y, N>) -> Self {
        Edges {
            l: l,
            finished: false,
            i: [0; N],
            j: 0,
            overlaps: ArrayVec::new(),
            l2: l,
            act: [pos!(0, 0); N],
            k: [0; N],
            k_fixed: [false; N]
        }
    }

    fn handle_overlaps(&mut self) -> Edge<X, Y, N> {
        //print!("handle_overlaps: {:?}\n", self);

        let mut result = self.l2;
        let mut mv = self.act;

        for (o, &k) in izip!(&self.overlaps, &self.k) {
            let a = o[k];
            o.iter().for_each(|&b| mv[b] = pos!(0, 0));
            mv[a] = self.act[a];
            result.pu[a] += mv[a];
        }

        let mut has_next = false;
        for (o, k, &k_fixed) in izip!(&self.overlaps, &mut self.k, &self.k_fixed) {
            if k_fixed {
                continue;
            }
            *k += 1;
            if *k >= o.len() {
                *k = 0;
            } else {
                has_next = true;
                break;
            }
        }
        if !has_next {
            self.overlaps.clear();
            self.k = [0; N];
            self.k_fixed = [false; N];
        }

        //print!("result={:?}, act={:?}\n\n", result, self.act);

        Edge::new(self.l, result, self.act)
    }
}

impl<const X: i8, const Y: i8, const N: usize> Iterator for Edges<X, Y, N> {
    type Item = Edge<X, Y, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.overlaps.is_empty() {
            return Some(self.handle_overlaps());
        }

        loop {
            let mut l = self.l;
            let mut act = act_zero();
            for a in 0..N {
                act[a] = MOVE[self.i[a]];
                l.pu[a] += act[a];
            }
            l.ev += MOVE[self.j];

            let mut has_next = false;
            for x in &mut self.i {
                *x = *x + 1;
                if *x >= MOVE.len() {
                    *x = 0;
                } else {
                    has_next = true;
                    break;
                }
            }
            if !has_next {
                self.j += 1;
                if self.j < MOVE.len() {
                    has_next = true;
                }
            }

            if !has_next {
                self.finished = true;
                return None;
            } else if in_bounds(l) {
                assert!(self.overlaps.is_empty());
                assert!(self.k == [0; N]);

                let mut flag = [false; N];
                for a in 0..N {
                    if flag[a] {
                        continue;
                    }

                    let same_loc: ArrayVec::<_, N> = (a..N)
                        .filter(|&b| l.pu[a] == l.pu[b]
                            && !flag[b])
                        .collect();

                    if same_loc.len() > 1 {
                        if let Some(k) = same_loc.iter().position(|&b| act[b] == pos!(0, 0)) {
                            self.k[self.overlaps.len()] = k;
                            self.k_fixed[self.overlaps.len()] = true;
                        }
                        
                        same_loc.iter().for_each(|&b| flag[b] = true);
                        self.overlaps.push(same_loc);
                    }
                }
                
                if self.overlaps.is_empty() {
                    return Some(Edge::new(self.l, l, act));
                } else {
                    //print!("overlaps, act={:?}, sets={1:?}\n", act, self.overlaps);
                    self.act = act;
                    self.l2.pu = self.l.pu;
                    self.l2.ev = l.ev;
                    for a in 0..N {
                        if !flag[a] {
                            self.l2.pu[a] += act[a];
                        }
                    }
                    return Some(self.handle_overlaps());
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SquareContents {
    None,
    Pursuer(u8),
    Evader,
    Capture
}

impl Display for SquareContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "."),
            Self::Pursuer(x) => write!(f, "{}", x),
            Self::Evader => write!(f, "e"),
            Self::Capture => write!(f, "X")
        }
    }
}

impl<const X: i8, const Y: i8, const N: usize> Display for Loc<X, Y, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xs, ys) = (X as usize, Y as usize);
        let mut grid = vec![vec![SquareContents::None; ys]; xs];
        
        for (i, a) in self.pu.iter().enumerate() {
            grid[a.x as usize][a.y as usize] = SquareContents::Pursuer(i as u8);
        }

        let evsquare = &mut grid[self.ev.x as usize][self.ev.y as usize];
        if *evsquare == SquareContents::None {
            *evsquare = SquareContents::Evader;
        } else {
            *evsquare = SquareContents::Capture;
        }
        
        write!(f, "{:?}", grid)
    }
}
