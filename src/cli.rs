use std::os::raw::c_uint;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Run the Pip Macro Utility
    Run {
        #[arg(long)]
        /// Input Key for Sys Pips. use key-repl to get Keycode
        in_sys: Option<c_uint>,
        #[arg(long)]
        /// Input Key for Eng Pips. use key-repl to get Keycode
        in_eng: Option<c_uint>,
        #[arg(long)]
        /// Input Key for Wep Pips. use key-repl to get Keycode
        in_wep: Option<c_uint>,
        #[arg(long)]
        /// Output Key for Sys Pips. use key-repl to get Keycode
        out_sys: Option<c_uint>,
        #[arg(long)]
        /// Output  Key for Eng Pips. use key-repl to get Keycode
        out_eng: Option<c_uint>,
        #[arg(long)]
        /// Output Key for Wep Pips. use key-repl to get Keycode
        out_wep: Option<c_uint>,
    },
    /// A helper to print keycodes for pressed keys
    KeyRepl,
}

#[derive(Parser)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
