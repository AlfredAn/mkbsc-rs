use std::fmt::Debug;
use array_init::from_iter;
use std::collections::BTreeSet;
use array_init::array_init;
use crate::algo::*;
use crate::algo::dkbsc::DKBSC;
use std::rc::Rc;
use crate::dgame::DGame;
use crate::game::*;
use crate::util::*;
use itertools::*;

type L<G, const N: usize> = <G as Game<N>>::Loc;
type D<G, const N: usize> = DGame<L<G, N>, N>;

type G<T, const N: usize> = DGame<T, N>;

type GI_<T, const N: usize> = Project<G<T, N>, Rc<G<T, N>>, N>;
type GILoc<T, const N: usize> = L<GI_<T, N>, 1>;
type GI<T, const N: usize> = D<GI_<T, N>, 1>;

type GKI_<T, const N: usize> = KBSC<GI<T, N>, Rc<GI<T, N>>>;
type GKILoc<T, const N: usize> = L<GKI_<T, N>, 1>;
type GKI<T, const N: usize> = D<GKI_<T, N>, 1>;

#[derive(Debug, Clone)]
pub struct MKBSC<T: Clone, const N: usize> {
    pub g: Rc<G<T, N>>,
    pub gi: [Rc<GI<T, N>>; N],
    pub gki: [DKBSC<GILoc<T, N>>; N],

    pub l0: [NodeIndex; N]
}

impl<T: Clone, const N: usize> MKBSC<T, N> {
    pub fn new(g: Rc<G<T, N>>) -> Self {
        let gi = array_init(|i|
            Rc::new(
                Project::<G<T, N>, _, N>(g.clone(), agent_index(i)).dgame()
            )
        );
        let gki = array_init(|i|
            DKBSC::new(gi[i].clone())
        );
        let l0 = array_init(|i|
            gki[i].gk.l0
        );

        Self {
            g, gi, gki, l0
        }
    }

    pub fn gki(&self, agt: usize) -> &GKI<T, N> {
        &self.gki[agt].gk
    }
}

impl<T: Clone + Debug, const N: usize> Game<N> for MKBSC<T, N> {
    type Loc = [NodeIndex; N];
    type Act = ActionIndex;
    type Obs = NodeIndex;
    type Agt = AgtIndex;

    fn l0(&self) -> &Self::Loc {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        izip!(n, &self.gki).any(|(s, gki)|
            gki.gk.is_winning(s)
        )
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> {
        let mut itr = n.iter()
            .enumerate()
            .map(|(i, &s)| &self.gki(i).node(s).data);
        let intersection = itr.next()
            .map(|set| itr.fold(set.clone(), |s1, s2| &s1 & &s2)).unwrap();
        let filter_set: BTreeSet<_> = self.g.post_set(intersection.iter(), a).collect();
        
        Box::new(iterator_product!(
            from_iter(
                izip!(n, a, &self.gki)
                    .map(|(n, a, k)| k.gk.post(n, [a]))
            ).unwrap()
        ).filter(move |s| {
            let mut itr = s.iter()
                .enumerate()
                .map(|(i, &s)| &self.gki(i).node(s).data);
            let intersection = itr.next()
                .map(|set| itr.fold(set.clone(), |s1, s2| &s1 & &s2)).unwrap();
            !filter_set.is_disjoint(&intersection)
        }))
    }

    fn actions(&self) -> Itr<[Self::Act; N]> {
        self.g.actions()
    }

    fn actions_i(&self, agt: Self::Agt) -> Itr<Self::Act> {
        self.g.actions_i(agt)
    }
    
    fn observe_i(&self, l: &Self::Loc, agt: Self::Agt) -> Self::Obs {
        l[agt.index()]
    }

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        *l
    }
}
