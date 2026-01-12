pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Plugin<T> {
    fn author(&self) -> &str;
    fn id(&self) -> &str;

    fn load(&self, ctx: &mut T) -> Result<()>;
}

pub struct ConcretePlugin<T> {
    author: String,
    id: String,
    loader: Box<dyn Fn(&mut T) -> Result<()>>, // Use Box<dyn Fn> instead of fn
    _library: Library,
}

impl<T> ConcretePlugin<T> {
    pub fn run(&self, ctx: &mut T) -> Result<()> {
        (self.loader)(ctx)
    }
}

// Re-export the macros
pub use cybird_macro::plugin;
use libloading::Library;
use std::ffi::*;
use std::path::Path;

pub fn try_load_libraries<P: AsRef<Path>>(directory: P) -> Result<Vec<Library>> {
    let mut result = vec![];
    let dir_path = directory.as_ref();

    // Check if directory exists
    if !dir_path.exists() {
        return Err("Directory does not exist".into());
    }

    if !dir_path.is_dir() {
        return Err("Not a directory".into());
    }

    let entries = std::fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            println!("File: {} (size: {} bytes)", path.display(), metadata.len());
            unsafe {
                result.push(Library::new(path)?);
            }
        } else if metadata.is_dir() {
            println!("Directory: {}", path.display());
        }
    }

    Ok(result)
}

pub fn try_into_plugins<T>(libraries: Vec<Library>) -> Result<Vec<ConcretePlugin<T>>> {
    let mut result = vec![];

    for lib in libraries {
        unsafe {
            // Get plugin metadata
            let get_author: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
                lib.get(b"get_author")?;
            let get_id: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
                lib.get(b"get_id")?;
            let load_plugin: libloading::Symbol<
                unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
            > = lib.get(b"load_plugin")?;
            let free_string: libloading::Symbol<unsafe extern "C" fn(*mut c_char)> =
                lib.get(b"free_string")?;

            // Get metadata strings
            let author_ptr = get_author();
            let id_ptr = get_id();
            let author = CStr::from_ptr(author_ptr).to_str()?.to_string();
            let id = CStr::from_ptr(id_ptr).to_str()?.to_string();

            println!("  Plugin Author: {}", author);
            println!("  Plugin ID: {}", id);

            // Create a safe wrapper function that captures the unsafe C function
            let load_fn = *load_plugin; // Copy the function pointer
            let safe_loader = Box::new(move |ctx: &mut T| -> Result<()> {
                let result = load_fn(ctx as *mut T as *mut std::ffi::c_void);
                if result == 0 {
                    Ok(())
                } else {
                    Err(format!("Plugin load failed with code: {}", result).into())
                }
            });

            // Free the allocated strings
            free_string(author_ptr as *mut c_char);
            free_string(id_ptr as *mut c_char);

            result.push(ConcretePlugin {
                author,
                id,
                loader: safe_loader,
                _library: lib,
            });
        }
    }

    Ok(result) // Return the actual result!
}
