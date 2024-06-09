use derive_more::{IsVariant, Unwrap};

#[derive(Debug, Copy, Clone, IsVariant, Unwrap)]
pub enum Either<L, D> {
    Living(L),
    Dead(D),
}

impl<L, D> Either<L, D> {
    #[inline]
    pub const fn as_ref(&self) -> Either<&L, &D> {
        match *self {
            Either::Living(ref living) => Either::Living(living),
            Either::Dead(ref dead) => Either::Dead(dead),
        }
    }

    #[inline]
    pub fn as_mut(&mut self) -> Either<&mut L, &mut D> {
        match *self {
            Either::Living(ref mut living) => Either::Living(living),
            Either::Dead(ref mut dead) => Either::Dead(dead),
        }
    }
}
