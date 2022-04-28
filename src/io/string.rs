use std::{rc::{Rc, Weak}, cell::RefCell, hash::Hash};

use weak_table::WeakHashSet;

use crate::PtrEqRc;

thread_local! {
    static GLOBAL_STRINGS: Interner = Interner::new();
}

pub fn intern(s: &str) -> Symbol {
    GLOBAL_STRINGS.with(|strings|
        strings.intern(s)
    )
}

#[derive(Debug, Default)]
pub struct Interner(RefCell<WeakHashSet<Weak<str>>>);

impl Interner {
    fn new() -> Self {
        Self::default()
    }

    fn intern(&self, s: &str) -> Symbol {
        let mut set = self.0.borrow_mut();
        if let Some(s) = set.get(s) {
            Symbol(PtrEqRc(s))
        } else {
            let rc = Rc::<str>::from(s);
            set.insert(rc.clone());
            Symbol(PtrEqRc(rc))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Symbol(PtrEqRc<str>);

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
