//! This is a CLI tool for controlling software PWM signals using the RPPAL library.
//!
#![deny(unsafe_code, missing_docs)]

mod log_macros;

use clap::Parser;
use core::fmt::Arguments;
use std::error::Error;

/// This trait defines the logging interface for the RppalSoftpwmTool.
pub trait RppalSoftpwmLog {
    fn output(self: &Self, args: Arguments);
    fn warning(self: &Self, args: Arguments);
    fn error(self: &Self, args: Arguments);
}

/// This struct represents the RppalSoftpwmTool.
pub struct RppalSoftpwmTool<'a> {
    log: &'a dyn RppalSoftpwmLog,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[repr(u8)]
enum BcmPin {
    Pin1 = 1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin23,
    Pin24,
    Pin25,
    Pin26,
    Pin27,
}

const DUTY_CYCLE_0_DEGREES: f64 = 2.5;
const DUTY_CYCLE_180_DEGREES: f64 = 12.5;
const DUTY_CYCLE_RANGE: f64 = DUTY_CYCLE_180_DEGREES - DUTY_CYCLE_0_DEGREES;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Disable colors in output
    #[arg(long = "no-color", short = 'n', env = "NO_CLI_COLOR")]
    no_color: bool,

    /// Frequency of the PWM signal in MHz
    #[arg(long = "frequency", short = 'f', default_value_t = 50)]
    frequency: u64,

    /// Sequence of BCM pins and angle values
    #[arg(long = "sequence", short = 's', value_parser = parse_pin_angle)]
    sequence: Vec<(BcmPin, f64)>,
}

fn parse_pin_angle(input: &str) -> Result<(BcmPin, f64), String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }
    let pin = parts[0]
        .parse::<BcmPin>()
        .map_err(|_| "Invalid pin".to_string())?;
    let angle = parts[1]
        .parse::<f64>()
        .map_err(|_| "Invalid angle".to_string())?;
    Ok((pin, angle))
}

impl<'a> RppalSoftpwmTool<'a> {
    pub fn new(log: &'a dyn RppalSoftpwmLog) -> RppalSoftpwmTool<'a> {
        RppalSoftpwmTool { log }
    }

    pub fn run(
        self: &mut Self,
        args: impl IntoIterator<Item = std::ffi::OsString>,
    ) -> Result<(), Box<dyn Error>> {
        let _cli = match Cli::try_parse_from(args) {
            Ok(m) => m,
            Err(err) => {
                output!(self.log, "{}", err.to_string());
                return Ok(());
            }
        };
        let mut latch_pin = Gpio::new()?.get(LATCH_PIN)?.into_output();

        latch_pin.set_reset_on_drop(false);

        fn degrees_to_duty_cycle(degrees: f64) -> f64 {
            (degrees * (DUTY_CYCLE_RANGE / 180.0) + DUTY_CYCLE_0_DEGREES) / 100.0
        }

        latch_pin.set_pwm_frequency(cli.frequency, degrees_to_duty_cycle(LATCH_DEGREES))?;
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        struct TestLogger;

        impl TestLogger {
            fn new() -> TestLogger {
                TestLogger {}
            }
        }

        impl RppalSoftpwmLog for TestLogger {
            fn output(self: &Self, _args: Arguments) {}
            fn warning(self: &Self, _args: Arguments) {}
            fn error(self: &Self, _args: Arguments) {}
        }

        let logger = TestLogger::new();
        let mut tool = RppalSoftpwmTool::new(&logger);
        let args: Vec<std::ffi::OsString> = vec!["".into(), "--help".into()];

        tool.run(args).unwrap();
    }
}
