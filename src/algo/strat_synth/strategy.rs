use crate::game::Game1;
use crate::Game;
use crate::MKBSC;
use crate::DGame;
use array_init::array_init;
use crate::ActionIndex;
use super::strategy1::*;

#[derive(Debug, Clone)]
pub struct AllStrategies<const N: usize> {
    parts: [AllStrategies1; N]
}

impl<const N: usize> AllStrategies<N> {
    pub fn advance(&mut self) -> bool {
        for p in &mut self.parts {
            if p.advance() {
                return true;
            } else {
                p.reset();
            }
        }

        false
    }

    pub fn get(&self) -> [&Vec<Option<ActionIndex>>; N] {
        array_init(|i| self.parts[i].get())
    }

    pub fn reset(&mut self) {
        for p in &mut self.parts {
            p.reset();
        }
    }

    fn new(parts: [AllStrategies1; N]) -> Self {
        Self { parts }
    }
}

pub fn all_strategies<const N: usize>(g: [&DGame<1>; N]) -> AllStrategies<N> {
    AllStrategies::new(
        g.map(|g| g.all_strategies())
    )
}

impl<'a, G, const N: usize> MKBSC<'a, G, N>
where
    G: Game<'a, N> + 'a,
    G::Loc: Ord
{
    pub fn all_strategies(&self) -> AllStrategies<N> {
        AllStrategies::new(
            array_init(|i| self.kbsc[i].dgame().all_strategies())
        )
    }
}
