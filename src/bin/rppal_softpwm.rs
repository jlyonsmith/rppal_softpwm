use core::fmt::Arguments;
use rppal_softpwm::{error, RppalSoftpwmLog, RppalSoftpwmTool};
use termion::color;

struct RppalSoftpwmLogger;

impl RppalSoftpwmLogger {
    fn new() -> RppalSoftpwmLogger {
        RppalSoftpwmLogger {}
    }
}

impl RppalSoftpwmLog for RppalSoftpwmLogger {
    fn output(self: &Self, args: Arguments) {
        println!("{}", args);
    }
    fn warning(self: &Self, args: Arguments) {
        eprintln!("{}warning: {}", color::Fg(color::Yellow), args);
    }
    fn error(self: &Self, args: Arguments) {
        eprintln!("{}error: {}", color::Fg(color::Red), args);
    }
}

fn main() {
    let logger = RppalSoftpwmLogger::new();

    if let Err(error) = RppalSoftpwmTool::new(&logger).run(std::env::args_os()) {
        error!(logger, "{}", error);
        std::process::exit(1);
    }
}
