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
//   File: src/api.rs
//   Desc: Api functions for usr_bin

// RustDoc
//! # ChadOS API
//!
//! This module provides the API functions for usr_bin in ChadOS.
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License
//!
//! # File: src/api.rs
//!
//! This file contains API functions for usr_bin

pub mod asm {
    //! Assembly acesss
    //!
    //! This module provides access to assembly-related features.
    pub use core::arch::{asm, global_asm};
}

pub mod alloc {
    //! Re-exports for memory allocations
    //!
    //! This module re-exports various items related to memory allocation.
    pub use alloc::{
        alloc::{alloc, dealloc, realloc},
        rc, vec,
    };
}

pub mod io {
    //! I/O functions for usr_bin
    //!
    //! This module provides I/O functions for usr_bin. Note that `kprintln` is privileged and is not exposed here.
    pub use crate::io::vga::prelude;
    pub use crate::{eprintln, print, println, rprint, wprintln};
}

pub mod time {
    //! Time related features and types
    //!
    //! This module provides access to time-related features and types.
    pub use crate::cfg;
    pub use crate::time::{sleep, Uptime, UptimeRepr};
}

pub mod scripting {
    //! Scripting utilities
    //!
    //! This module provides scripting utilities for usr_bin. It includes types, macros, and functions for scripting purposes.

    use alloc::string::String;

    // Represents the result of a command execution.
    pub type CmdRes = Result<Option<String>, String>;

    // Represents the command arguments.
    pub type CmdArgs<'a> = &'a [&'a str];

    // Macros used for command registration and argument parsing.
    pub use crate::{console::FUNCS, parse, register};

    // Macros

    /// Parse a value from a string, returning a result.
    ///
    /// This macro attempts to parse a value of the specified type from a string. If parsing fails, it returns an error with a message.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = parse!("42", i32);
    /// assert_eq!(result, Ok(42));
    /// ```
    #[macro_export]
    macro_rules! parse {
        ($arg:expr, $type:ty) => {
            $arg.parse::<$type>().map_err(|_| {
                alloc::format!(
                    "Invalid argument. Expected {}, Recieved {}.",
                    stringify!($type),
                    $arg
                )
            })
        };
    }

    /// Register a function in the console.
    ///
    /// This macro is used to register a function in the console, allowing it to be called by name.
    ///
    /// # Examples
    ///
    /// ```
    /// register!(FUNCS, my_function);
    /// ```
    #[macro_export]
    macro_rules! register {
        ($map:ident, $func:expr) => {
            $map.insert(stringify!($func), $func as $crate::console::CmdFn)
        };
    }
}
