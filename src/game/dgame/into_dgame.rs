use std::fmt::Debug;
use arrayvec::ArrayVec;
use std::cell::RefCell;
use std::collections::*;
use super::*;
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Clone)]
pub struct FromGameResult<G: Game<N> + ?Sized, const N: usize> {
    pub dg: DGame<G::Loc, N>,

    pub dl_map: Vec<G::Loc>,
    pub l_map: HashMap<G::Loc, NodeIndex>,

    #[default(array_init(|_| Vec::new()))]
    pub do_map: [Vec<G::Obs>; N],

    #[default(array_init(|_| HashMap::new()))]
    pub o_map: [HashMap<G::Obs, ObsIndex>; N],

    #[default(array_init(|_| Vec::new()))]
    pub da_map: [Vec<G::Act>; N],

    #[default(array_init(|_| HashMap::new()))]
    pub a_map: [HashMap<G::Act, ActionIndex>; N]
}

pub fn into_dgame<G: Game<N> + ?Sized, const N: usize>(g: &G) -> FromGameResult<G, N> {
    let c = RefCell::new(FromGameResult::<G, N>::default());
    {
        let mut c = c.borrow_mut();
        for (i, agt) in (0..N).map(|i| (i, G::agent(i))) {
            for a in g.actions_i(agt) {
                let da = action_index(c.da_map[i].len());
                c.da_map[i].push(a);
                let old = c.a_map[i].insert(a, da);
                assert!(old.is_none());
            }
        }
        c.dg.n_actions = c.da_map.iter()
            .map(|m| m.len())
            .max()
            .unwrap();
    }

    println!("\n{:?}", g);

    explore(
        g,
        |l| {
            println!("{:?}", &l);

            let mut c = c.borrow_mut();

            let dl = node_index(c.dg.node_count());

            let obs = g.observe(l);
            let mut dobs = ArrayVec::<_, N>::new();
            for i in 0..N {
                let o;
                if let Some(&_o) = c.o_map[i].get(&obs[i]) {
                    o = _o;
                } else {
                    o = obs_index(c.do_map[i].len());

                    c.do_map[i].push(obs[i].clone());
                    c.o_map[i].insert(obs[i].clone(), o);

                    c.dg.obs[i].push(DObs::default());

                    assert_eq!(c.do_map[i].len(), c.dg.obs[i].len());
                };

                c.dg.obs[i][o.index()].set.push(dl);
                dobs.push(o);
            }
            assert_eq!(dobs.len(), N);

            let dl2 = c.dg.graph.add_node(DNode::new(
                g.is_winning(l),
                (*dobs).try_into().unwrap(),
                l.clone()
            ));
            assert_eq!(dl, dl2);

            c.dl_map.push(l.clone());

            let old = c.l_map.insert(l.clone(), dl);
            assert!(old.is_none());
        },
        |l, a, l2| {
            println!("{:?}", (&l, a, &l2));

            let mut c = c.borrow_mut();

            let (dl, dl2) = (c.l_map[l], c.l_map[l2]);
            let da = array_init(|i| c.a_map[i][&a[i]]);

            if !c.dg.graph.edges_connecting(dl, dl2).any(|e| e.weight().act == da) {
                c.dg.graph.add_edge(dl, dl2, DEdge::new(da));
            }
        }
    );

    {
        let mut c = c.borrow_mut();
        assert!(c.dg.node_count() > 0);
        c.dg.l0 = node_index(0);
    }

    c.into_inner()
}
