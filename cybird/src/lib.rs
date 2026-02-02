pub mod prelude {
    // Core types and traits
    pub use crate::{Context, Plugin, Result};
    pub use crate::{FromRegistrable, FromRegistrableMut};

    // Derive macros
    pub use cybird_macro::{Context, Registrable, plugin};
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Plugin<T: Context> {
    fn author(&self) -> &str;
    fn id(&self) -> &str;

    fn load(&self, ctx: &mut T) -> Result<()>;
}

pub trait Context {
    type Registrable;

    fn register<T>(&mut self, registrable: T)
    where
        T: Into<Self::Registrable>;

    fn get_registrables<T>(&self) -> Vec<&T>
    where
        T: FromRegistrable<Self::Registrable>;

    fn get_registrables_mut<T>(&mut self) -> Vec<&mut T>
    where
        T: FromRegistrableMut<Self::Registrable>;
}

pub trait FromRegistrable<R> {
    fn from_registrable(registrable: &R) -> Option<&Self>;
}

pub trait FromRegistrableMut<R> {
    fn from_registrable_mut(registrable: &mut R) -> Option<&mut Self>;
}
