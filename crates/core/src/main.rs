use javy::Runtime;
use once_cell::sync::{OnceCell};
use std::io::{self, Read};
use std::slice;
use std::str;
use std::string::String;

mod execution;
mod runtime;

const FUNCTION_MODULE_NAME: &str = "function.mjs";

static mut RUNTIME: Vec<Runtime> = vec![];
static mut BYTECODE: Vec<u8> = vec![];


#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    // Read the file permissions from an env var
    let file_permissions = std::env::var("FILE_PERMISSIONS");
    let runtime = match file_permissions {
        Ok(env) => {
            //eprintln!("Reading permissions from env var: {}", env);
            // parse as yaml
            let file_permissions = serde_yaml::from_str(&env);
            match file_permissions {
                Ok(file_permissions) => runtime::new_runtime_with_file_permissions(file_permissions).unwrap(),
                Err(e) => {
                    eprintln!("Invalid permissions file: {}", e);
                    runtime::new_runtime().unwrap()
                }
            }
        
        }
        Err(e) => {
            eprintln!("Could not read file permissions from env var: {}", e);
            runtime::new_runtime().unwrap()
        }
    };

    // Read the http permissions from an env var
    let http_permissions = std::env::var("HTTP_PERMISSIONS");
    let runtime = match http_permissions {
        Ok(env) => {
            //eprintln!("Reading permissions from env var: {}", env);
            // parse as yaml
            let http_permissions = serde_yaml::from_str(&env);
            match http_permissions {
                Ok(http_permissions) => runtime::new_runtime_with_http_permissions(http_permissions).unwrap(),
                Err(e) => {
                    eprintln!("Invalid permissions http: {}", e);
                    runtime::new_runtime().unwrap()
                }
            }
        
        }
        Err(e) => {
            eprintln!("Could not read file permissions from env var: {}", e);
            runtime::new_runtime().unwrap()
        }
    };
   
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents).unwrap();

    let bytecode = runtime
        .context()
        .compile_module("function.mjs", &contents)
        .unwrap();

    unsafe {
        RUNTIME = vec![runtime];
        BYTECODE = bytecode;
    }
}

fn main() {
    //let bytecode = unsafe { BYTECODE };
    //let runtime = unsafe { RUNTIME };
    unsafe { execution::run_bytecode(&RUNTIME[0], &BYTECODE) };
}

// Removed in post-processing.
/// Evaluates QuickJS bytecode and invokes the exported JS function name.
///
/// # Safety
///
/// * `fn_name_ptr` must reference a UTF-8 string with `fn_name_size` byte
///   length.
#[export_name = "javy.invoke"]
pub unsafe extern "C" fn invoke(fn_name_ptr: *mut u8, fn_name_size: usize) {
    let js_fn_name = str::from_utf8_unchecked(slice::from_raw_parts(fn_name_ptr, fn_name_size));
    execution::invoke_function(unsafe { &RUNTIME[0] }, FUNCTION_MODULE_NAME, js_fn_name);
}