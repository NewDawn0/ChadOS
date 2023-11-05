//     ____ _               _  ___  ____
//    / ___| |__   __ _  __| |/ _ \/ ___|
//   | |   | '_ \ / _` |/ _` | | | \___ \
//   | |___| | | | (_| | (_| | |_| |___) |
//    \____|_| |_|\__,_|\__,_|\___/|____/
//    https://github.com/NewDawn0/ChadOS
//
//   @Author: NewDawn0
//   @Contributors: -
//   @License: MIT
//
//   File: src/usr_bin/mod.rs
//   Desc: usr_bin module file

// RustDoc
//! # ChadOS User Bin Module
//!
//! This module contains functions that are part of the ChadOS user bin, which provides a set of built-in
//! commands and utilities that users can run in the operating system.
//!
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License. See the MIT License section below for details.
//!
//! # File: src/usr_bin/mod.rs
//!
//! This file defines user bin functions for ChadOS.

// Imports
use crate::{
    api::{
        io::println,
        scripting::{parse, register, CmdArgs, CmdRes, FUNCS},
        time::Uptime,
    },
    io::vga::clear_all,
};
use alloc::string::{String, ToString};

/// Initalizes all user functions
pub fn init() {
    let mut funcs = FUNCS.write();
    register!(funcs, echo);
    register!(funcs, tee);
    register!(funcs, uptime);
    register!(funcs, clear);
    register!(funcs, sum);
}

// @NOTE: A user function needs to have the function signature fn(CmdArgs) -> CmdRes otherwise it will not register
// @NOTE: CmdArgs is a type alias for `&[&str]`
// @NOTE: CmdRes is a type alias for `Result<Option<String>, String>`
// @NOTE: To help parse the arg from a &str to whatever type use `parse!(<arg>, <type>)?;`

// Example functions

/// Example function: echo
///
/// This function echoes the input arguments as a string.
///
/// # Arguments
///
/// - `args`: A slice of `&str` representing the arguments to echo.
///
/// # Return
///
/// Returns `Ok(None)` if successful.
fn echo(args: CmdArgs) -> CmdRes {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
    Ok(None)
}

/// Example function: tee
///
/// This function echoes the input arguments as a string and returns them.
///
/// # Arguments
///
/// - `args`: A slice of `&str` representing the arguments to tee.
///
/// # Return
///
/// Returns `Ok(Some(String))` containing the echoed input arguments.
fn tee(args: CmdArgs) -> CmdRes {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
    Ok(Some(args.join(" ")))
}

/// Example function: uptime
///
/// This function returns the uptime information, such as the number of seconds or ticks since system boot.
///
/// # Arguments
///
/// - `args` (optional): An optional specifier to request specific uptime information.
///
/// # Return
///
/// Returns `Ok(Some(String))` containing the requested uptime information as a string.
fn uptime(args: CmdArgs) -> CmdRes {
    let res = match args.is_empty() {
        true => Uptime::string_fmt(),
        false => {
            match args[0] {
                "-s" => Uptime::secs().to_string(),  // Use seconds
                "-t" => Uptime::ticks().to_string(), // Use ticks
                "-d" => Uptime::string_fmt(),        // Use default
                _ => {
                    return Err(String::from("Invalid specifier"));
                }
            }
        }
    };
    Ok(Some(res))
}

/// Example function: clear
///
/// This function clears the console.
///
/// # Return
///
/// Returns `Ok(None)` if successful.
fn clear(args: CmdArgs) -> CmdRes {
    if !args.is_empty() {
        return Err("Clear takes no arguments".to_string());
    }
    clear_all(); //HACK: NOT AN API SHOULD NOT BE USED OUTSIDE OF THIS FUNCION
    Ok(None)
}

/// Example function: sum
///
/// This function calculates the sum of a list of integers.
///
/// # Arguments
///
/// - `args`: A slice of integers to be summed.
///
/// # Return
///
/// Returns `Ok(Some(String))` containing the sum as a string.
fn sum(args: CmdArgs) -> CmdRes {
    if args.len() < 2 {
        return Err("Usage: add <i32> <i32> ...".to_string());
    }
    let mut res = 0;
    for arg in args {
        res += parse!(arg, i32)?;
    }
    Ok(Some(res.to_string()))
}
