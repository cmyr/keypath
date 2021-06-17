use std::marker::PhantomData;

/// A component of a keypath.
#[derive(Debug, Clone, Copy)]
pub enum PathComponent {
    Unnamed(usize),
    Named(&'static str),
    IndexInt(usize),
    IndexStr(&'static str),
}

#[derive(Debug, Clone, Copy)]
pub struct PathChecker<T>(PhantomData<*const T>);

impl<T: super::TypedKeyable> PathChecker<T> {
    pub fn get(self) -> T::PathFragment {
        T::get()
    }
}

impl<T> Default for PathChecker<T> {
    fn default() -> Self {
        PathChecker(PhantomData)
    }
}
