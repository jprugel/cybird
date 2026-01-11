pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Plugin<T> {
    fn author(&self) -> &str;
    fn id(&self) -> &str;

    fn load(&self, ctx: &mut T) -> Result<()>;
}

// Re-export the macros
pub use cybird_macro::plugin;
