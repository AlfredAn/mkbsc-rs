use std::hash::Hash;

trait Game<Agt, Loc, Act, Obs>
where
    Agt: Agent,
    Loc: Location,
    Act: Action,
    Obs: Observation<Agt, Loc>
{
    fn l0(&self) -> Loc;
    fn agents(&self) -> &[Agt];
    fn neighbors(&self, l: &Loc) -> Vec<Neighbor<Loc, Act>>;
}

trait Agent: Eq {}

trait Location: Eq + Hash {}

trait Action: Eq + Hash {}

trait Observation<Agt: Agent, Loc: Location>: Eq + Hash {
    fn observe(a: &Agt, l: &Loc) -> Self;
    fn locations(&self) -> Vec<Loc>;
    fn contains(&self, l: &Loc) -> bool;
}

trait ReachObjective<Loc: Location> {
    fn is_achieved<I: Iterator<Item=Loc>>(history: I) -> bool;
}

#[derive(PartialEq, Eq, Hash)]
struct Neighbor<Loc: Location, Act: Action> {
    l: Loc,
    a: Vec<Act>
}
