use crate::types::Float;

pub(crate) trait Math {
    fn square(self) -> Float;
}

impl Math for Float {
    /// A clear, fast way that LLVM should be able to optimize for computing x.powi(2)
    #[inline]
    fn square(self) -> Float {
        self * self
    }
}
