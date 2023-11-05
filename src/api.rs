pub mod asm {
    pub use core::arch::{asm, global_asm};
}
pub mod alloc {
    pub use alloc::{
        alloc::{alloc, dealloc, realloc},
        rc, vec,
    };
}
pub mod io {
    // No kprintln as is privileged
    pub use crate::io::vga::prelude;
    pub use crate::{eprintln, print, println, rprint, wprintln};
}
pub mod time {
    pub use crate::cfg;
    pub use crate::time::{sleep, Uptime, UptimeRepr};
}
pub mod scripting {
    use alloc::string::String;
    pub type CmdRes = Result<Option<String>, String>;
    pub type CmdArgs<'a> = &'a [&'a str];
    // Macros
    pub use crate::{console::FUNCS, parse, register};
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
    #[macro_export]
    macro_rules! register {
        ($map:ident, $func:expr) => {
            $map.insert(stringify!($func), $func as $crate::console::CmdFn)
        };
    }
}
