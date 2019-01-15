use std::marker::PhantomData;

pub(crate) fn phantom_data<T>(_: T) -> PhantomData<T> {
    PhantomData
}
