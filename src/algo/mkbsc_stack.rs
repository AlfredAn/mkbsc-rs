use crate::*;

/*
pub struct MKBSCStack<const N: usize> {
    base: Rc<Game<Rc<dyn Data>, N>>,
    stack: Vec<Rc<Game<MKBSCData<Rc<dyn Data>, N>, N>>>
}

impl<const N: usize> MKBSCStack<N> {
    pub fn new(base: Rc<Game<Rc<dyn Data>, N>>) -> Self {
        Self {
            base, stack: Vec::new()
        }
    }

    pub fn last(&self) -> Option<&Rc<Game<MKBSCData<Rc<dyn Data>, N>, N>>> {
        self.stack.last()
    }

    pub fn push(&mut self) {
        if let Some(g) = self.last() {
            let gk = MKBSC::new(g.clone());
            let gk = gk.build_with(|d: MKBSCData<_, N>| {
                let r = d as MKBSCData<Rc<dyn Data>, N>;
                r
            });
            self.stack.push(gk.game);
        } else {
            let gk = MKBSC::new(self.base.clone()).build();
            self.stack.push(gk.game);
        }
    }
}

*/
