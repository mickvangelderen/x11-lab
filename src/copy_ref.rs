use std::marker::PhantomData;
use std::ptr;
use super::freeze::Freeze;
use super::phantom_data::phantom_data;

/// To enable implementing Copy, we need users of CopyRef to provide a
/// copyable type that can hold all valid bit patterns of `T`. With the
/// `Raw` trait we promise that we can take Self, store it in a Raw and
/// interpret an immutable reference to Raw as an immutable reference to
/// Self.
pub unsafe trait Elegible: Freeze {
    type Raw: Copy;

    fn as_raw(&self) -> &Self::Raw;
}

/// https://users.rust-lang.org/t/references-to-values-smaller-than-references/21448/5
#[repr(transparent)]
#[derive(Debug)]
pub struct CopyRef<'a, T>
where
    T: Elegible,
{
    copy: T::Raw,
    _borrow: PhantomData<&'a T>,
}

impl<'a, T> CopyRef<'a, T>
where
    T: Elegible,
{
    #[inline]
    pub fn new(value: &'a T) -> Self {
        use std::mem::size_of;

        assert_eq!(size_of::<T>(), size_of::<T::Raw>());

        let raw = value.as_raw();

        assert_eq!(value as *const T, raw as *const T::Raw as *const T);

        unsafe {
            CopyRef {
                copy: ptr::read(raw),
                _borrow: phantom_data(value),
            }
        }
    }
}

// NOTE: Deriving Clone does not work for all types while it should.
impl<'a, T> Clone for CopyRef<'a, T>
where
    T: Elegible,
{
    #[inline]
    fn clone(&self) -> Self {
        CopyRef {
            copy: self.copy,
            _borrow: self._borrow,
        }
    }
}

// NOTE: Deriving Copy does not work for all types while it should.
impl<'a, T> Copy for CopyRef<'a, T> where T: Elegible {}

impl<'a, T> std::ops::Deref for CopyRef<'a, T>
where
    T: Elegible,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // Safe because we
        // 1. hold an immutable borrow,
        // 2. Self::Target has no interior mutability, and
        // 3. all valid bit patterns of T::Raw are valid for T.
        unsafe { &*(&self.copy as *const T::Raw as *const T) }
    }
}

#[cfg(test)]
mod tests {
    use super::Elegible;
    use super::CopyRef;

    struct Resource(u32);

    impl Resource {
        fn as_u32(&self) -> u32 {
            self.0
        }
    }

    unsafe impl Elegible for Resource {
        type Raw = u32;
    }

    #[test]
    fn it_is_indeed_smaller() {
        use std::mem::size_of;

        assert!(size_of::<CopyRef<Resource>>() < size_of::<&Resource>());
    }

    #[test]
    fn can_copy() {
        let x = Resource(13);
        let r = CopyRef::new(&x);
        let r2 = r;
        let r3 = r;
        assert_eq!(r2.as_u32(), 13);
        assert_eq!(r3.as_u32(), 13);
    }
}
