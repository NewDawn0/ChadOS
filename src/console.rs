use crate::{
    api::scripting::{CmdArgs, CmdRes},
    cfg::console::{CMD_ERR_COL, CMD_OK_COL, CMD_OUT_COL, CMD_SEPERATOR},
    io::vga::clear_char,
    io::vga::prelude::*,
    keys::{Modifiers, KEY_HANDLER},
    usr_bin,
};
use alloc::{string::String, vec::Vec};
use core::sync::atomic::{AtomicBool, Ordering};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::RwLock;

static OK_CMD: AtomicBool = AtomicBool::new(true);
// Type
pub type CmdFn = fn(CmdArgs) -> CmdRes;
lazy_static! {
    pub static ref FUNCS: RwLock<HashMap<&'static str, CmdFn>> = RwLock::new(HashMap::new());
    pub static ref CMD_LINE: RwLock<String> = RwLock::new(String::new());
}

// Init
pub fn init() {
    // Hijack KEY_HANDLER with custom key_handler
    kprintln!("[CONSOLE] Setting handler");
    let mut handler = KEY_HANDLER.write();
    *handler = key_handler;
    // Inialize user functions
    kprintln!("[CONSOLE] Initalizing custom functions");
    usr_bin::init();
}
fn key_handler(c: char, mods: Modifiers) {
    let mut cmdline = CMD_LINE.write();
    if mods.clear {
        clear_char();
        cmdline.pop();
    } else {
        cmdline.push(c);
        print!("{}", c)
    }
    drop(cmdline);
    // Detect submit
    if mods.enter {
        submit();
        CMD_LINE.write().clear();
        match OK_CMD.load(Ordering::Relaxed) {
            true => COL.lock().set_fg(CMD_OK_COL),
            false => COL.lock().set_fg(CMD_ERR_COL),
        }
        print!("> "); // print console init
        COL.lock().set_default();
    }
}

fn submit() {
    let input = &CMD_LINE.read(); // Clone to avoid holding the lock
    let cmds: Vec<&str> = input.trim().split(CMD_SEPERATOR).collect();
    let mut prev_out: Option<String> = None;
    // check for empty lines
    if !cmds.is_empty() && !cmds[0].is_empty() {
        for (index, cmd) in cmds.iter().enumerate() {
            let cmdbits: Vec<&str> = cmd.split_whitespace().collect();
            let cmd = cmdbits[0];
            let mut args = cmdbits[1..].to_vec();
            if let Some(ref out) = prev_out {
                args.extend(out.split_whitespace().collect::<Vec<&str>>())
            }
            let (cmdout, sig) = exec(cmd, &args, &FUNCS.read());
            match sig {
                Signal::None => {} // Ignore
                Signal::Break => {
                    // Exit this command
                    print_cmd_res(cmdout);
                    break;
                }
            }
            check_out(&mut prev_out, cmdout);
            if index == cmds.len() - 1 {
                print_last_cmd_res(&prev_out)
            }
        }
    }
}

// Utils
enum Signal {
    Break,
    // Exit,
    None,
}

#[inline]
fn print_cmd_res(res: CmdRes) {
    match res {
        Ok(a) => println!("{}", a.unwrap()),
        Err(e) => println!("{}", e),
    }
}
#[inline]
fn print_last_cmd_res(prev_out: &Option<String>) {
    if let Some(ref out) = prev_out {
        COL.lock().set_fg(CMD_OUT_COL);
        println!("<< {}", out);
        COL.lock().set_default();
    }
}

#[inline]
fn exec(cmd: &str, args: CmdArgs, funcs: &HashMap<&str, CmdFn>) -> (CmdRes, Signal) {
    match cmd {
        "list" => {
            println!("Available commands:");
            for func in funcs.keys() {
                println!(" - {}", func);
            }
            OK_CMD.store(true, Ordering::Relaxed);
            (Ok(None), Signal::None)
        }
        _ => {
            if let Some(cmd) = funcs.get(cmd) {
                match cmd(args) {
                    Ok(out) => {
                        OK_CMD.store(true, Ordering::Relaxed);
                        (Ok(out), Signal::None)
                    }
                    Err(e) => {
                        OK_CMD.store(false, Ordering::Relaxed);
                        (Err(e), Signal::Break)
                    }
                }
            } else {
                OK_CMD.store(false, Ordering::Relaxed);
                (Err(String::from("Command not found")), Signal::Break)
            }
        }
    }
}

#[inline]
fn check_out(prev_out: &mut Option<String>, cmdout: Result<Option<String>, String>) {
    match cmdout {
        Ok(out) => {
            *prev_out = out.clone();
        }
        Err(e) => {
            println!("{}", e);
            // Some error during command
            *prev_out = None;
        }
    }
}
