use std::marker::PhantomData;

pub fn phantom_data<T>(_: T) -> PhantomData<T> {
    PhantomData
}
