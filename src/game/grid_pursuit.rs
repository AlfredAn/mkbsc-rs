use derive_more::*;
use itertools::{izip, chain};

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Sub, AddAssign, SubAssign)]
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
pub struct GridPursuitGame<const N: usize> {
    l0: Loc<N>,
    max: Pos
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc<const N: usize> { l: RawLoc<N>, capture: bool }

impl Pos {
    pub fn vis(self) -> impl Iterator<Item=Pos> {
        VIS.into_iter()
            .map(move |delta| self + delta)
    }

    pub fn obs(self, other: Pos) -> Option<u8> {
        self.vis()
            .find_position(|&vis| vis == other)
            .map(|(i, _)| i.try_into().unwrap())
    }

    pub fn in_bounds(self, max: Pos) -> bool {
        (0..max.x).contains(&self.x) && (0..max.y).contains(&self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, new)]
pub struct RawLoc<const N: usize> {
    pu: [Pos; N],
    ev: Pos,
    max: Pos
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Obs<const N: usize> {
    this: Pos,
    pu: [Option<u8>; N],
    ev: Option<u8>,
    capture: bool
}

impl<const N: usize> Loc<N> {
    pub fn is_winning(self) -> bool {
        self.l.is_winning() || self.capture
    }

    pub fn obs(self, agt: Agt) -> Obs<N> {
        let this = self.l.pu[agt.index()];

        let pu = from_iter(
            self.l.pu.into_iter()
            .map(|other| this.obs(other))
        ).unwrap();
        
        let ev = this.obs(self.l.ev);

        Obs { this, pu, ev, capture: self.capture }
    }
}

impl<const N: usize> Add for RawLoc<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            array_init(|i| self.pu[i] + rhs.pu[i]),
            self.ev + rhs.ev,
            self.max
        )
    }
}

impl<const N: usize> Sub for RawLoc<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            array_init(|i| self.pu[i] - rhs.pu[i]),
            self.ev - rhs.ev,
            self.max
        )
    }
}

impl<const N: usize> AddAssign for RawLoc<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<const N: usize> SubAssign for RawLoc<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

type Sym = (u8, bool);

const SYM: [Sym; 8] = [
    (0, false), (1, false), (2, false), (3, false),
    (0, true), (1, true), (2, true), (3, true)
];

fn sym(mut p: Pos, (rot, flip): Sym, max: Pos) -> Pos {
    if flip {
        p = pos!(max.x-1-p.x, p.y);
    }

    match rot & 3 {
        0 => p,
        1 => pos!(p.y, max.x-1-p.x),
        2 => pos!(max.x-1-p.x, max.y-1-p.y),
        3 => pos!(max.y-1-p.y, p.x),
        _ => unreachable!()
    }
}

fn sym_act(mut p: Pos, (rot, flip): Sym) -> Pos {
    if flip {
        p = pos!(-p.x, p.y);
    }

    match rot & 3 {
        0 => p,
        1 => pos!(p.y, -p.x),
        2 => pos!(-p.x, -p.y),
        3 => pos!(-p.y, p.x),
        _ => unreachable!()
    }
}

impl<const N: usize> RawLoc<N> {
    pub fn is_winning(self) -> bool {
        self.pu.contains(&self.ev)
    }

    fn sym_private(self, s: Sym) -> Self {
        Self {
            pu: array_init::array_init(|a| sym(self.pu[a], s, self.max)),
            ev: sym(self.ev, s, self.max),
            max: self.max
        }
    }

    pub fn sym(self) -> impl Iterator<Item=Self> {
        iter::once(self)
        // SYM.into_iter()
        //     .map(move |s| self.sym_private(s))
    }

    // pub fn view(self, agt: Agt, capture: bool) -> (Sym, Obs<N>) {
    //     let (i, _) = self.sym()
    //         .enumerate()
    //         .min_by_key(|(_, l)| (l.pu[agt.index()], l.pu, l.ev))
    //         .unwrap();
    //     let sym = SYM[i];
        
    //     let this = self.pu[agt.index()];
    //     let pu = from_iter(
    //         self.pu.into_iter()
    //         .map(|other| this.obs(other))
    //     ).unwrap();
    //     let ev = this.obs(self.ev);

    //     let obs = Obs::<N> { this, pu, ev, capture };

    //     todo!()
    // }

    pub fn canonical(self) -> Self {
        // self.sym().min().unwrap()
        self
    }

    pub fn to_loc(self, capture: bool) -> Loc<N> {
        Loc {
            l: self.canonical(),
            capture: if self.is_winning() { false } else { capture }
        }
    }

    pub fn in_bounds(self) -> bool {
        self.pu.into_iter()
            .all(|pu| pu.in_bounds(self.max))
          && self.ev.in_bounds(self.max)
    }

    pub fn fmt_compact(&self) -> String {
        if N != 2 {
            todo!();
        // } else if self.is_winning() {
        //     "W".into()
        } else {
            format!("{}{}{}{}{}{}", self.pu[0].x, self.pu[0].y, self.pu[1].x, self.pu[1].y, self.ev.x, self.ev.y)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, new)]
pub struct Edge<const N: usize> {
    from: Loc<N>,
    to: Loc<N>,
    act: Action<N>
}

type Action<const N: usize> = [Pos; N];

const fn act_zero<const N: usize>() -> Action<N> {
    [pos!(0, 0); N]
}

impl GridPursuitGame<2> {
    pub fn new(xsize: i8, ysize: i8) -> Self {
        let max = pos!(xsize, ysize);
        Self {
            l0: RawLoc {
                pu: [pos!(xsize-1, 0), pos!(0, ysize-1)],
                ev: pos!(0, 0), max
            }.to_loc(false),
            max
        }
    }
}

impl<const N: usize> AbstractGame<N> for GridPursuitGame<N> {
    type Loc = Loc<N>;
    type Obs = Obs<N>;

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
        array_init(|i| l.obs(agt(i)))
    }

    fn succ(
        &self,
        l: &Self::Loc,
        mut f: impl FnMut([Act; N], Self::Loc)
    ) {
        cartesian_product([&MOVE; N], |p_move| {
            let p_move = p_move.map(|&x| x);
            let p_old = l.l.pu;
            let p_new = izip!(p_old, p_move)
                .map(|(old, mv)| old + mv)
                .collect_array::<N>()
                .unwrap();

            if p_new.into_iter().any(|p| !p.in_bounds(self.max)) {
                return;
            }

            let a = p_move.map(|mv| act(
                MOVE.into_iter()
                    .find_position(|&x| x == mv)
                    .unwrap()
                    .0
            ));

            let overlap: ArrayVec<_, N> = p_new.into_iter()
                .enumerate()
                .filter(|&(i, this)|
                    chain!(
                        p_old.into_iter().enumerate(),
                        p_new.into_iter().enumerate()
                    )
                    .any(|(j, other)| i != j && this == other)
                )
                .map(|(i, _)| i)
                .collect();
                
            assert!(N <= 32);
            for k in 0..1u32 << overlap.len() {
                let mut p_final = p_new;
                for i in 0..overlap.len() {
                    if k & (1 << i) == 0 {
                        p_final[overlap[i]] = p_old[overlap[i]];
                    }
                }
                if !p_final.into_iter()
                    .enumerate()
                    .any(|(i, this)|
                        p_final[i+1..].contains(&this)
                    ) 
                    && overlap.iter()
                        .all(|&i| p_final.contains(&p_new[i])) {
                        for e_move in MOVE {
                            let e_old = l.l.ev;
                            let e_new = e_old + e_move;
            
                            if !e_new.in_bounds(self.max) { continue; }
            
                            let result = RawLoc::new(
                                p_final,
                                e_new,
                                self.max
                            ).to_loc((0..N)
                                .any(|i|
                                       p_old[i] == e_new
                                    && p_final[i] == e_old
                                )
                            );

                            f(a, result);
                        }
                    }
            }
        });
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, l: &Self::Loc) -> fmt::Result {
        write!(f, "{}{}",
            l.l.fmt_compact(),
            if l.l.is_winning() { "W" } else { if l.is_winning() { "X" } else { "" } }
        )
    }

    fn fmt_act(&self, f: &mut fmt::Formatter, a: Act) -> fmt::Result {
        write!(f, "{}", a)
    }
}

fn in_bounds<const N: usize>(l: RawLoc<N>) -> bool {
    chain!(
        l.pu.into_iter(),
        iter::once(l.ev)
    ).all(|p|
        (0..l.max.x).contains(&p.x)
     && (0..l.max.y).contains(&p.y)
    )
}

/*#[derive(Clone, Debug)]
pub struct Edges<const N: usize> {
    l: Loc<N>,
    finished: bool,
    i: [usize; N],
    j: usize,
    overlaps: ArrayVec<ArrayVec<usize, N>, N>,
    l2: RawLoc<N>,
    act: Action<N>,
    k: [usize; N],
    k_fixed: [bool; N]
}



fn overlaps<const N: usize>(l: RawLoc<N>) -> bool {
    for (i, ai) in l.pu.iter().enumerate() {
        for aj in l.pu.iter().skip(i+1) {
            if ai == aj {
                return true;
            }
        }
    }
    false
}

impl<const N: usize> Edges<N> {
    fn new(l: Loc<N>) -> Self {
        Edges {
            l: l,
            finished: false,
            i: [0; N],
            j: 0,
            overlaps: ArrayVec::new(),
            l2: l.l,
            act: [pos!(0, 0); N],
            k: [0; N],
            k_fixed: [false; N]
        }
    }

    fn handle_overlaps(&mut self) -> Edge<N> {
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

        let result = result.to_loc(
            result.pu.into_iter()
                .contains(&self.l.l.ev)
        );

        Edge::new(self.l, result, self.act)
    }
}

impl<const N: usize> Iterator for Edges<N> {
    type Item = Edge<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if !self.overlaps.is_empty() {
            return Some(self.handle_overlaps());
        }

        loop {
            let mut l = self.l.l;
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
                    let result = l.to_loc(
                        l.pu.into_iter()
                            .contains(&self.l.l.ev)
                    );
                    return Some(Edge::new(self.l, result, act));
                } else {
                    //print!("overlaps, act={:?}, sets={1:?}\n", act, self.overlaps);
                    self.act = act;
                    self.l2.pu = self.l.l.pu;
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

impl<const N: usize> Display for RawLoc<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xs, ys) = (self.max.x as usize, self.max.y as usize);
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
}*/
