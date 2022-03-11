use {
    core::fmt,
    log::{self, Level, LevelFilter, Log, Metadata, Record},
};

use core::fmt::{Arguments, Result, Write};
use crate::sbi::console_putchar;

// use core::arch::asm;

use spin::Mutex;

struct Console;

fn putchar(c: u8) {
    super::sbi::console_putchar(c as usize);
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            if c == 127 {
                putchar(8);
                putchar(b' ');
                putchar(8);
            } else {
                putchar(c);
            }
        }
        Ok(())
    }
}

pub fn putfmt(fmt: Arguments) {
    static CONSOLE: Mutex<Console> = Mutex::new(Console);
    CONSOLE.lock().write_fmt(fmt).unwrap();
}




pub fn init() {
    log::set_logger(&SimpleLogger).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::logging::print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

pub fn print_in_color(args: fmt::Arguments, color_code: u8) {
    putfmt(with_color!(args, color_code));
}



pub fn print(args: fmt::Arguments) {
    putfmt(args);
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

            print_in_color(
                format_args!(
                    "[{:>5}][{},-] {}\n",
                    record.level(),
                    hart_id(),
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        }
    
    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 34,  // BrightYellow
        Level::Info => 33,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}



pub fn hart_id() -> usize {
    let hart_id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) hart_id);
    }
    hart_id
}

// pub fn print_colorized(args: fmt::Arguments, foreground_color: u8, background_color: u8) {
//     CONSOLE.lock().write_fmt(colorize!(args, foreground_color, background_color)).unwrap();
// }

// #[macro_export]
// macro_rules! print_colorized {
//     ($fmt: literal, $foreground_color: expr, $background_color: expr $(, $($arg: tt)+)?) => {
//         $crate::console::print_colorized(format_args!($fmt $(, $($arg)+)?), $foreground_color as u8, $background_color as u8);
//     };
// }

// #[macro_export]
// macro_rules! println_colorized {
//     ($fmt: literal, $foreground_color: expr, $background_color: expr $(, $($arg: tt)+)?) => {
//         $crate::console::print_colorized(format_args!(concat!($fmt, "\r\n") $(, $($arg)+)?), $foreground_color as u8, $background_color as u8);
//     }
// }

// #[macro_export]
// macro_rules! println_hart {
//     ($fmt: literal, $hart_id: expr $(, $($arg: tt)+)?) => {
//         $crate::console::print_colorized(format_args!(concat!("[hart {}]", $fmt, "\r\n"), $hart_id $(, $($arg)+)?), 93 + $hart_id as u8, 49u8);
//     };
// }