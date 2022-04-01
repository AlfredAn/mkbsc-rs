/*macro_rules! rc {
    ($x:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($x))
    };
    ($($x:expr),*) => {
        std::rc::Rc::new((
            $(std::cell::RefCell::new($x)),*
        ))
    };
}
pub(crate) use rc;*/
