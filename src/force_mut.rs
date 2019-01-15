// note(mickvangelderen): don't expose this outside of the crate, super dangerous.
pub(crate) trait ForceMut {
    /// Don't understand the implications of doing this but I want to force my
    /// pointers to be mutable sometimes to be able to pass them to external
    /// functions.
    unsafe fn force_mut(&self) -> &mut Self;
}

impl<T> ForceMut for T {
    unsafe fn force_mut(&self) -> &mut Self {
        &mut *(self as *const Self as *mut Self)
    }
}
