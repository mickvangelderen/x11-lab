use std::cell::UnsafeCell;
use std::marker::PhantomData;

/// Copied from https://doc.rust-lang.org/1.28.0/src/core/marker.rs.html#584
/// Compiler-internal trait used to determine whether a type contains
/// any `UnsafeCell` internally, but not through an indirection.
/// This affects, for example, whether a `static` of that type is
/// placed in read-only static memory or writable static memory.
pub unsafe auto trait Freeze {}

impl<T: ?Sized> !Freeze for UnsafeCell<T> {}
unsafe impl<T: ?Sized> Freeze for PhantomData<T> {}
unsafe impl<T: ?Sized> Freeze for *const T {}
unsafe impl<T: ?Sized> Freeze for *mut T {}
unsafe impl<'a, T: ?Sized> Freeze for &'a T {}
unsafe impl<'a, T: ?Sized> Freeze for &'a mut T {}

