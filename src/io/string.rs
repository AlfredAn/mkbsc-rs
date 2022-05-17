use std::rc::Weak;

use crate::*;
use weak_table::WeakHashSet;

thread_local! {
    static GLOBAL_STRINGS: Interner<str> = Interner::default();
}

pub fn intern(s: &str) -> Symbol {
    GLOBAL_STRINGS.with(|strings|
        Symbol(strings.intern(s))
    )
}

#[derive(Debug)]
pub struct Interner<T: ?Sized + Eq + Hash>(RefCell<WeakHashSet<Weak<T>, BuildHasherDefault<FxHasher>>>);

impl<T: ?Sized + Eq + Hash> Default for Interner<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Eq + Hash + Clone> Interner<T> {
    pub fn intern(&self, s: &T) -> Interned<T> {
        let mut set = self.0.borrow_mut();
        if let Some(s) = set.get(s) {
            PtrEqRc(s)
        } else {
            let rc = Rc::new(s.clone());
            set.insert(rc.clone());
            PtrEqRc(rc)
        }
    }
}

impl Interner<str> {
    pub fn intern(&self, s: &str) -> Interned<str> {
        let mut set = self.0.borrow_mut();
        if let Some(s) = set.get(s) {
            PtrEqRc(s)
        } else {
            let rc = Rc::from(s);
            set.insert(Rc::clone(&rc));
            PtrEqRc(rc)
        }
    }
}

pub type Interned<T> = PtrEqRc<T>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(Interned<str>);

impl Clone for Symbol {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.0)
    }
}
