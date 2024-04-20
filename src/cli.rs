use std::os::raw::c_uint;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
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

        #[arg(long)]
        /// Toggle-Switch that triggers on press/depress. use key-repl to get Keycode
        in_landing_gear: Option<c_uint>,
        #[arg(long)]
        /// The landing gear button that gets pressed. use key-repl to get Keycode
        out_landing_gear: Option<c_uint>,

        #[arg(long, short)]
        /// Set this is you want to use the landing gear feature. If in-landing-gear OR out-landing-gear is set, this is not needed
        use_gear: bool,
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
