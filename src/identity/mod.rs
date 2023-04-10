use std::fmt::Debug;

pub trait Identity: Debug {
    fn id(&self) -> &'static str;
    fn __id() -> &'static str
    where
        Self: Sized;
}
