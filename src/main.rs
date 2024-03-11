use clap::Parser;
mod cli;
mod commands;
mod keycodes;

use commands::run::{run_command, Settings};
use keycodes::key_from_code;
use rdev::{listen, Key};

use crate::{
    cli::{Cli, Commands},
    keycodes::code_from_key,
};

fn main() {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or(Commands::Run {
        in_sys: None,
        in_eng: None,
        in_wep: None,
        out_sys: None,
        out_eng: None,
        out_wep: None,
    });

    match command {
        Commands::KeyRepl => {
            eprintln!("Entering REPL mode. Use this to figure out which Key has which keycode");
            if let Err(err) = listen(move |x| {
                let pressed_key = match x.event_type {
                    rdev::EventType::KeyPress(ev) => ev,
                    _ => return,
                };

                let key_str = match code_from_key(pressed_key) {
                    Some(val) => format!("{:?}", val),
                    None => String::from("None"),
                };

                println!(
                    "Pressed key {:?}, which has Keycode {:?}",
                    pressed_key, key_str
                );
            }) {
                eprintln!("Err: {:?}", err);
            }
        }
        Commands::Run {
            in_sys,
            in_eng,
            in_wep,
            out_sys,
            out_eng,
            out_wep,
        } => {
            let in_sys = in_sys.map_or(Key::Num1, key_from_code);
            let out_sys = out_sys.map_or(Key::LeftArrow, key_from_code);
            let in_eng = in_eng.map_or(Key::Num2, key_from_code);
            let out_eng = out_eng.map_or(Key::UpArrow, key_from_code);
            let in_wep = in_wep.map_or(Key::Num3, key_from_code);
            let out_wep = out_wep.map_or(Key::RightArrow, key_from_code);

            run_command(Settings {
                sys: (in_sys, out_sys),
                eng: (in_eng, out_eng),
                wep: (in_wep, out_wep),
            })
        }
    }
}
