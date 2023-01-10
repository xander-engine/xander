use std::any::Any;

pub trait Identity {
    fn id(&self) -> &'static str;
}

pub(crate) trait ToAny : 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<I : Identity + ?Sized + 'static> ToAny for &'static I {
    fn as_any(&self) -> &dyn Any {
        self
    }
}