use std::io::Write;

use itertools::chain;
use indoc::{writedoc};
use tabbycat::GraphBuilder;
use crate::{*, cli::Format};

#[derive(Clone, SmartDefault)]
pub struct Game<const N: usize> {
    pub loc: Vec<LocData<N>>,

    #[default([0; N])]
    pub n_actions: [usize; N],

    #[default(array_init(|_| Vec::new()))]
    pub obs: [Vec<Vec<Loc>>; N],

    pub origin: Option<Rc<dyn Origin>>
}

#[derive(Debug, Clone)]
pub struct LocData<const N: usize> {
    // sorted by action
    pub successors: Vec<([Act; N], Loc)>, 
    pub predecessors: Vec<([Act; N], Loc)>,
    
    pub is_winning: bool,
    pub obs: [Obs; N],
    pub obs_offset: [usize; N]
}

impl<const N: usize> Game<N> {
    pub fn n_loc(&self) -> usize {
        self.loc.len()
    }
    pub fn n_obs(&self, agt: Agt) -> usize {
        self.obs[agt.index()].len()
    }

    pub fn observe(&self, l: Loc) -> [Obs; N] {
        self[l].obs
    }
    pub fn obs_offset(&self, l: Loc) -> [usize; N] {
        self[l].obs_offset
    }
    pub fn is_winning(&self, l: Loc) -> bool {
        self[l].is_winning
    }

    pub fn obs_set(&self, agt: Agt, obs: Obs) -> &[Loc] {
        &self.obs[agt.index()][obs.index()]
    }

    pub fn n_action_profiles(&self) -> usize {
        self.n_actions.iter().product()
    }
    pub fn action_profiles(&self) -> impl Iterator<Item=[Act; N]> {
        range_product(self.n_actions.map(|n| 0..n))
            .map(|aa| aa.map(|a| act(a)))
    }
    fn action_profile_index(&self, a: [Act; N]) -> usize {
        let mut result = 0;
        for i in (0..N).rev() {
            result *= self.n_actions[i];
            result += a[i].index();
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item=(Loc, &LocData<N>)> {
        self.loc.iter().enumerate().map(|(i, x)| (loc(i), x))
    }
    pub fn iter_obs(&self, agt: Agt) -> impl Iterator<Item=(Obs, &[Loc])> {
        self.obs[agt.index()].iter().enumerate().map(|(i, x)| (obs(i), &**x))
    }
    pub fn iter_agt(&self) -> impl Iterator<Item=Agt> + Clone {
        (0..N).map(|i| agt(i))
    }
    pub fn iter_act(&self, agt: Agt) -> impl Iterator<Item=Act> {
        (0..self.n_actions[agt.index()]).map(|i| act(i))
    }

    pub fn edges(&self) -> impl Iterator<Item=(Loc, [Act; N], Loc)> + '_ {
        self.iter()
            .flat_map(|(l, d)|
                d.successors.iter()
                    .map(move |&(a, l2)| (l, a, l2))
            )
    }

    pub fn edges_dedup(&self) -> impl Iterator<Item=(Loc, Vec<[Act; N]>, Loc)> + '_ {
        self.iter()
            .flat_map(|(l, d)| {
                d.successors.iter()
                    .sort_and_group_by_key(|(_, l2)| l2)
                    .into_iter()
                    .map(|(l2, g)| (
                            l,
                            g.map(|&(a, _)| a)
                                .collect(),
                            *l2
                        ))
                    .collect_vec()
                    .into_iter()
            })
    }

    pub fn successors(&self, l: Loc) -> &[([Act; N], Loc)] {
        &self[l].successors
    }
    pub fn post_raw(&self, l: Loc, a: [Act; N]) -> &[([Act; N], Loc)] {
        find_group(self.successors(l), |x|
            a.cmp(&x.0)
        )
    }
    pub fn post(&self, l: Loc, a: [Act; N]) -> impl Iterator<Item=Loc> + '_ {
        self.post_raw(l, a).iter().map(|&(_, l)| l)
    }
    pub fn post_set<'a, I>(&'a self, s: I, a: [Act; N]) -> impl Iterator<Item=Loc> + 'a
    where
        I: IntoIterator<Item=Loc>,
        I::IntoIter: 'a
    {
        s.into_iter()
            .flat_map(move |l|
                self.post(l, a)
            )
    }
    pub fn succ_set<'a, I>(&'a self, s: I) -> impl Iterator<Item=([Act; N], Loc)> + 'a
    where
        I: IntoIterator<Item=Loc>,
        I::IntoIter: 'a
    {
        s.into_iter()
            .flat_map(move |l|
                self.successors(l)
                    .iter()
                    .copied()
            )
    }

    pub fn predecessors(&self, l: Loc) -> &[([Act; N], Loc)] {
        &self[l].predecessors
    }
    pub fn pre_raw(&self, l: Loc, a: [Act; N]) -> &[([Act; N], Loc)] {
        find_group(self.predecessors(l), |x|
            a.cmp(&x.0)
        )
    }
    pub fn pre(&self, l: Loc, a: [Act; N]) -> impl Iterator<Item=Loc> + '_ {
        self.pre_raw(l, a).iter().map(|&(_, l)| l)
    }
    pub fn pre_set<'a, I>(&'a self, s: I, a: [Act; N]) -> impl Iterator<Item=Loc> + 'a
    where
        I: IntoIterator<Item=Loc>,
        I::IntoIter: 'a
    {
        s.into_iter()
            .flat_map(move |l|
                self.pre(l, a)
            )
    }

    pub fn l0(&self) -> Loc {
        loc(0)
    }

    pub fn obs0(&self) -> [Obs; N] {
        self.observe(self.l0())
    }

    pub fn is_obs_winning(&self, agt: Agt, obs: Obs) -> bool {
        self.obs_set(agt, obs)
            .iter()
            .all(|&l| self.is_winning(l))
    }

    pub fn neighbors(&self, l: Loc) -> impl Iterator<Item=Loc> + '_ {
        self.successors(l)
            .iter()
            .map(|&(_, l)| l)
            .dedup()
    }

    pub fn to_unique_loc(&self, obs: Obs, agt: Agt) -> Option<Loc> {
        if self.obs[agt.index()][obs.index()].len() == 1 {
            Some(self.obs[agt.index()][obs.index()][0])
        } else {
            None
        }
    }

    pub fn fmt_loc(&self, f: &mut fmt::Formatter, l: Loc) -> fmt::Result {
        if let Some(origin) = &self.origin {
            origin.fmt_loc(f, l)
        } else {
            write!(f, "{}", l)
        }
    }

    pub fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, obs: Obs) -> fmt::Result {
        format_sep(f, "|", self.obs_set(agt, obs).iter(), |f, &l|
            self.fmt_loc(f, l)
        )
    }
}

impl<const N: usize> Index<Loc> for Game<N> {
    type Output = LocData<N>;
    fn index(&self, l: Loc) -> &LocData<N> { &self.loc[l.index()] }
}
impl<const N: usize> IndexMut<Loc> for Game<N> {
    fn index_mut(&mut self, l: Loc) -> &mut LocData<N> { &mut self.loc[l.index()] }
}

impl<const N: usize> Debug for Game<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {{\n")?;
        write!(f, "    n_agents: {}, n_actions: {:?},\n", N, self.n_actions)?;
        write!(f, "    Loc: {{ ")?;
        format_sep(f, ",\n           ", self.iter(), |f, (l, _)|
            self.fmt_loc(f, l)
        )?;

        for agt in self.iter_agt() {
            write!(f, " }},\n    Obs[{}]: {{ ", agt)?;
            format_sep(f, ",\n              ", self.iter_obs(agt), |f, (o, _)|
                self.fmt_obs(f, agt, o)
            )?;
        }
        
        write!(f, " }},\n    Delta: {{ ")?;
        format_sep(f, ",\n             ", self.edges(), |f, (l, a, l2)| {
            format_sep(f, ".", a.iter(), |f, a|
                write!(f, "{}", a)
            )?;
            write!(f, " : ")?;
            self.fmt_loc(f, l)?;
            write!(f, " -> ")?;
            self.fmt_loc(f, l2)
        })?;

        write!(f, " }}\n}}")
    }
}

impl<const N: usize> Display for Game<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "agt: ")?;
        format_sep(f, ", ",
            0..N,
            |f, i| write!(f, "{}", i)
        )?;

        write!(f, "\nact: ")?;
        format_sep(f, ", ",
            0..self.n_actions.into_iter().max().unwrap(),
            |f, i| write!(f, "{}", i)
        )?;

        write!(f, "\nloc: ")?;
        format_sep(f, ", ",
            self.iter(),
            |f, (l, _)| self.fmt_loc(f, l)
        )?;

        write!(f, "\nl0: ")?;
        self.fmt_loc(f, self.l0())?;

        let mut reach = self.iter()
            .filter(|(_, data)| data.is_winning);

        if let Some(first) = reach.next() {
            write!(f, "\nreach: ")?;
            format_sep(f, ", ",
                chain!(iter::once(first), reach),
                |f, (l, _)| self.fmt_loc(f, l)
            )?;
        }

        for i in 0..N {
            let agt = agt(i);

            let mut obs = self.iter_obs(agt)
                .filter(|(_, set)| set.len() > 1);

            if let Some(first) = obs.next() {
                write!(f, "\nobs {}: ", agt)?;
                format_sep(f, ", ",
                    chain!(iter::once(first), obs),
                    |f, (_, set)| format_sep(f, "|",
                        set.iter(),
                        |f, &l| self.fmt_loc(f, l)
                    )
                )?;
            }
        }

        write!(f, "\ndelta:\n")?;
        format_sep(f, ",\n",
            self.edges(),
            |f, (l, a, l2)| {
                self.fmt_loc(f, l)?;
                format_sequence(f,
                    SequenceFormat {
                        start: " (",
                        sep: " ",
                        end: ") "
                    },
                    a.into_iter(),
                    |f, a| write!(f, "{}", a)
                )?;
                self.fmt_loc(f, l2)
            }
        )?;

        Ok(())
    }
}

fn tex_escaped(f: &mut fmt::Formatter, input: &str) -> fmt::Result {
    for c in input.chars() {
        match c {
            '&'|'%'|'$'|'#'|'_'|'{'|'}' => write!(f, r"\{c}"),
            '~' => write!(f, r"\textasciitilde "),
            '^' => write!(f, r"\textasciicircum "),
            '\\' => write!(f, r"\textbackslash "),
            _ => write!(f, "{c}")
        }?;
    }
    Ok(())
}

impl<const N: usize> Game<N> {
    pub fn format(&self, f: &mut fmt::Formatter, fmt: &Format) -> fmt::Result {
        match fmt {
            Format::Default => write!(f, "{}", self),
            Format::Tikz => self.format_tikz(f),
            Format::Dot => self.format_dot(f),
        }
    }

    pub fn format_tikz(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nodes = display(|f| {
            for (l, data) in self.iter() {
                let label = display(|f|
                    tex_escaped(f,
                        &format!("{}",
                            display(|f| self.fmt_loc(f, l))
                        )
                    )
                );

                let options = display(|f| {
                    write!(f, "state")?;
                    if self.l0() == l { write!(f, ", initial")?; }
                    if data.is_winning { write!(f, ", accepting")?; }
                    Ok(())
                });

                let name = l.index();
                writeln!(f, r"    \node[{options}] ({name}) {{{label}}};")?;
            }

            Ok(())
        });

        println!("{:?}", self.iter().map(|x| x.0).collect_vec());
        println!("{:?}", self.edges().collect_vec());

        let edges = display(|f| {
            for ((l, l2), group) in self.edges().sort_and_group_by_key(|&(l, _, l2)| (l, l2)).into_iter() {
                let label = display_once(|f|
                    format_sep(f, ",", group, |f, (_, a, _)|
                        format_sequence(f, SequenceFormat {
                            start: "(", end: ")", sep: ","
                        }, a.iter(), |f, ai|
                            write!(f, "a{ai}")
                        )
                    )
                );

                let options = display(|f| {
                    // write!(f, "above")?;
                    if l == l2 {
                        write!(f, "loop above")?;
                    }
                    Ok(())
                });
                let l2 = display(|f| if l != l2 { write!(f, "{l2}") } else { Ok(()) });

                writeln!(f, r"    \draw ({l}) edge[{options}] node{{{label}}} ({l2});")?;
            }

            Ok(())
        });

        let obs = display(|f| {
            for (l, _) in self.iter() {
                let o = self.observe(l);
                for (l2, _) in self.iter().skip(l.index()+1) {
                    if l == l2 { continue; }

                    let o2 = self.observe(l2);

                    let mut agt = self.iter_agt()
                        .filter(|agt| o[agt.index()] == o2[agt.index()])
                        .peekable();

                    if let Some(_) = agt.peek() {
                        let options = "obs";
                        let label = display(|f|
                            format_sep(f, ",", agt.clone(), |f, agt| write!(f, "{agt}"))
                        );

                        writeln!(f, r"    \draw ({l}) edge[{options}] node{{\textasciitilde {label}}} ({l2});")?;
                    }
                }
            }
            Ok(())
        });

        writedoc!(f, r"
            \begin{{figure}}
              \centering
              \begin{{tikzpicture}}
            {nodes}
            {edges}
            {obs}
              \end{{tikzpicture}}
              \caption{{Game}}
              \label{{fig:game}}
            \end{{figure}}
        ")?;

        Ok(())
    }

    pub fn format_dot(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use tabbycat::*;

        let idt = |l|
            Identity::quoted(format!("{}", display(|f| self.fmt_loc(f, l))));

        let graph = GraphBuilder::default()
            .graph_type(GraphType::DiGraph)
            .strict(false)
            .id(Identity::id("Game").unwrap())
            .stmts({
                let mut list = StmtList::new();

                for (l, _) in self.iter() {
                    list = list.add_node(
                        idt(l),
                        None,
                        None
                    );
                }

                for (l, a, l2) in self.edges() {
                    list = list.add_edge(
                        Edge::head_node(idt(l), None)
                            .arrow_to_node(idt(l2), None)
                    );
                }

                list
            })
            .build()
            .unwrap();

        write!(f, "{}", graph)
    }
}
