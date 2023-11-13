use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "javy", about = "JavaScript to WebAssembly toolchain")]
pub enum Command {
    /// Compiles JavaScript to WebAssembly.
    Compile(CompileCommandOpts),
    /// Emits the provider binary that is required to run dynamically
    /// linked WebAssembly modules.
    EmitProvider(EmitProviderCommandOpts),
}

#[derive(Debug, StructOpt)]
pub struct CompileCommandOpts {
    #[structopt(parse(from_os_str))]
    /// Path of the JavaScript input file.
    pub input: PathBuf,

    #[structopt(short = "o", parse(from_os_str), default_value = "index.wasm")]
    /// Desired path of the WebAssembly output file.
    pub output: PathBuf,

    #[structopt(short = "d")]
    /// Creates a smaller module that requires a dynamically linked QuickJS provider Wasm
    /// module to execute (see `emit-provider` command).
    pub dynamic: bool,

    #[structopt(long = "wit")]
    /// Optional path to WIT file describing exported functions and fs/http permissions
    /// Only supports function exports with no arguments and no return values.
    pub wit: Option<PathBuf>,

    #[structopt(long = "file-permissions")]
    /// Optional path to WIT file describing exported functions and fs/http permissions
    /// Only supports function exports with no arguments and no return values.
    pub fpermissions: Option<PathBuf>,


    #[structopt(long = "http-permissions")]
    /// Optional path to WIT file describing exported functions and fs/http permissions
    /// Only supports function exports with no arguments and no return values.
    pub httppermissions: Option<PathBuf>,

    #[structopt(short = "n")]
    /// Optional WIT world name for WIT file. Must be specified if WIT is file path is specified.
    pub wit_world: Option<String>,

    #[structopt(long = "remove-node-red-send")]
    pub remove_nred_send: bool,

    #[structopt(long = "remove-node-red-done")]
    pub remove_nred_done: bool,

    #[structopt(long = "remove-node-red-warn")]
    pub remove_nred_warn: bool,

    #[structopt(long = "remove-node-red-error")]
    pub remove_nred_error: bool,

    #[structopt(long = "remove-node-red-emit")]
    pub remove_nred_emit: bool,

    #[structopt(long = "remove-node-red-pop")]
    pub remove_nred_pop: bool,

    #[structopt(long = "remove-node-red-node")]
    pub remove_nred_node: bool,

    #[structopt(long = "remove-node-red-msg")]
    pub remove_nred_msg: bool,


    #[structopt(long = "remove-node-red-register")]
    pub remove_nred_register: bool,

    #[structopt(long = "remove-node-red-result")]
    pub remove_nred_result: bool,


    #[structopt(long = "remove-fs")]
    pub remove_fs: bool,
}

#[derive(Debug, StructOpt)]
pub struct EmitProviderCommandOpts {
    #[structopt(long = "out", short = "o")]
    /// Output path for the provider binary (default is stdout).
    pub out: Option<PathBuf>,
}
