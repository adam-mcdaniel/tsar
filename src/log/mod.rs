pub mod build;
pub mod color;

use std::fmt::{Display, Error, Formatter};
pub struct LazyPrint<A: Display, B: Display, C: Display>(A, B, C);

impl<A: Display, B: Display, C: Display> LazyPrint<A, B, C> {
    pub fn new(a: A, b: B, c: C) -> Self {
        Self(a, b, c)
    }
}

impl<A: Display, B: Display, C: Display> Display for LazyPrint<A, B, C> {
    fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
        let Self(before, data, after) = self;
        print!("{}{}{}", before, data, after);
        Ok(())
    }
}
