use crate::{
    api::{
        io::println,
        scripting::{parse, register, CmdArgs, CmdRes, FUNCS},
        time::Uptime,
    },
    io::vga::clear_all,
};
use alloc::string::{String, ToString};

// Initalize user functions
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

/// Example function: echo
/// @ARG: &[&str]
/// @RVAL: None
fn echo(args: CmdArgs) -> CmdRes {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
    Ok(None)
}

/// Example function: tee
/// @ARG: &[&str]
/// @RVAL: &[&str]
fn tee(args: CmdArgs) -> CmdRes {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" "));
    }
    Ok(Some(args.join(" ")))
}

/// Example function: uptime
/// @ARG [optional]:
/// @RVAL: Uptime
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
/// @RVAL: None
fn clear(args: CmdArgs) -> CmdRes {
    if !args.is_empty() {
        return Err("Clear takes no arguments".to_string());
    }
    clear_all(); //HACK: NOT AN API SHOULD NOT BE USED OUTSIDE OF THIS FUNCION
    Ok(None)
}

/// Example function: sum
/// @ARG: &[i32]
/// @RVAL: String::from(i32)
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
