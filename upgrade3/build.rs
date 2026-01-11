use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    if env::var("CARGO_FEATURE_DYNAMIC").is_ok() {
        println!("cargo:rustc-cfg=dynamic_library");
        analyze_and_generate_exports();
    }
}

fn analyze_and_generate_exports() {
    let lib_rs_path = Path::new("src/lib.rs");
    if let Ok(content) = fs::read_to_string(lib_rs_path) {
        let exports = if content.contains("impl Plugin<") {
            let (struct_name, context_type) = extract_plugin_info(&content);
            generate_exports_for_plugin(&struct_name, &context_type)
        } else {
            String::new()
        };

        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("auto_exports.rs");

        fs::write(&dest_path, exports).unwrap();
        println!("cargo:rustc-env=AUTO_EXPORTS_PATH={}", dest_path.display());
    }
}

fn extract_plugin_info(content: &str) -> (String, String) {
    let mut struct_name = "UnknownPlugin".to_string();
    let mut context_type = "()".to_string();

    for line in content.lines() {
        if line.contains("pub struct") && !line.contains("//") {
            if let Some(name) = line.split_whitespace().nth(2) {
                struct_name = name.trim_end_matches(';').to_string();
                break;
            }
        }
    }

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("impl Plugin<") && trimmed.contains(&format!("for {}", struct_name))
        {
            // Parse: impl Plugin<SomeType> for StructName
            if let Some(start) = trimmed.find("Plugin<") {
                let after_bracket = &trimmed[start + 7..]; // Skip "Plugin<"
                if let Some(end) = after_bracket.find('>') {
                    context_type = after_bracket[..end].trim().to_string();
                    println!("Context Type: {}", context_type);
                }
            }
            break;
        }
    }

    (struct_name, context_type)
}

fn generate_exports_for_plugin(struct_name: &str, context_type: &str) -> String {
    format!(
        r#"
use std::os::raw::c_char;
use std::ffi::CString;

#[unsafe(no_mangle)]
pub extern "C" fn get_author() -> *const c_char {{
    let plugin = {struct_name}::default();
    let author = plugin.author();
    let c_string = CString::new(author).expect("CString::new failed");
    c_string.into_raw()
}}

#[unsafe(no_mangle)]
pub extern "C" fn get_id() -> *const c_char {{
    let plugin = {struct_name}::default();
    let id = plugin.id();
    let c_string = CString::new(id).expect("CString::new failed");
    c_string.into_raw()
}}

#[unsafe(no_mangle)]
pub extern "C" fn load_plugin(ctx_ptr: *mut std::ffi::c_void) -> i32 {{
    unsafe {{
        if ctx_ptr.is_null() {{
            return -1; // Error: null pointer
        }}

        // Cast the void pointer back to the expected type
        let ctx = &mut *(ctx_ptr as *mut {context_type});

        let plugin = {struct_name}::default();
        match plugin.load(ctx) {{
            Ok(_) => 0,  // Success
            Err(_) => -1, // Error
        }}
    }}
}}

// Cleanup function to free the allocated strings
#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {{
    unsafe {{
        if !s.is_null() {{
            let _ = CString::from_raw(s);
        }}
    }}
}}
"#,
        struct_name = struct_name,
        context_type = context_type
    )
}
