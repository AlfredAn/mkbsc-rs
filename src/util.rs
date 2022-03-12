pub trait CustomIterator: Iterator {
    
}

impl<I> CustomIterator for I where I: Iterator {}
