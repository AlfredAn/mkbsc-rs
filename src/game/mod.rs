use std::hash::Hash;

trait Game<Agt: Agent, Loc: Location, Act: Action, Obs: Observation<Loc>> {
    fn l0() -> Loc;
    fn neighbors(l: &Loc) -> &Vec<Neighbor<Loc, Act>>;
}

trait Agent {}

trait Location: Eq + Hash {}

trait Action: Eq + Hash {}

trait Observation<Loc: Location>: Eq + Hash {
    fn new(l: &Loc) -> Self;
    fn locations(&self) -> &Vec<Loc>;
    fn contains(&self, l: &Loc) -> bool;
}

trait Objective<Loc: Location> {
    fn is_achieved<I: Iterator<Item=Loc>>(history: I) -> bool;
}

#[derive(PartialEq, Eq, Hash)]
struct Neighbor<Loc: Location, Act: Action> {
    l: Loc,
    a: Vec<Act>
}
