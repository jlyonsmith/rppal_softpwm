//! This is a CLI tool for controlling software PWM signals using the RPPAL library.
//!
#![deny(unsafe_code, missing_docs)]

mod log_macros;

use clap::{Parser, ValueEnum};
use core::fmt::Arguments;
use rppal::gpio::Gpio;
use std::{error::Error, time::Duration};

/// This trait defines the logging interface for the RppalSoftpwmTool.
pub trait RppalSoftpwmLog {
    /// Output a message to the log.
    fn output(self: &Self, args: Arguments);
    /// Output a warning message to the log.
    fn warning(self: &Self, args: Arguments);
    /// Output an error message to the log.
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

    /// BCM pin to use for PWM output
    #[arg(long = "pin", short = 'p')]
    pin: BcmPin,

    /// Frequency of the PWM signal in Hz
    #[arg(long = "frequency", short = 'f', default_value_t = 50)]
    frequency: u64,

    /// Sequence of angles in degrees and times in milliseconds
    #[arg(long = "angles", short = 'a', value_name = "ANGLE:TIME", value_parser = parse_angle_time, value_delimiter = ',', num_args = 1..)]
    angles: Vec<(u64, u64)>,
}

fn parse_angle_time(s: &str) -> Result<(u64, u64), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }
    let angle = parts[0].parse().map_err(|_| "Invalid angle".to_string())?;

    if angle > 180 {
        return Err("Angle must be between 0 and 180".to_string());
    }

    let time = parts[1].parse().map_err(|_| "Invalid time".to_string())?;
    Ok((angle, time))
}

impl<'a> RppalSoftpwmTool<'a> {
    /// Create a new instance of the tool.
    pub fn new(log: &'a dyn RppalSoftpwmLog) -> RppalSoftpwmTool<'a> {
        RppalSoftpwmTool { log }
    }

    /// Run the tool with the given arguments.
    pub fn run(
        self: &mut Self,
        args: impl IntoIterator<Item = std::ffi::OsString>,
    ) -> Result<(), Box<dyn Error>> {
        let cli = match Cli::try_parse_from(args) {
            Ok(m) => m,
            Err(err) => {
                output!(self.log, "{}", err.to_string());
                return Ok(());
            }
        };
        let mut latch_pin = Gpio::new()?.get(cli.pin as u8)?.into_output();

        latch_pin.set_reset_on_drop(false);

        fn degrees_to_duty_cycle(degrees: f64) -> f64 {
            (degrees * (DUTY_CYCLE_RANGE / 180.0) + DUTY_CYCLE_0_DEGREES) / 100.0
        }

        for (angle, time) in cli.angles {
            output!(self.log, "{}° for {} ms", angle, time);
            latch_pin
                .set_pwm_frequency(cli.frequency as f64, degrees_to_duty_cycle(angle as f64))?;
            std::thread::sleep(Duration::from_millis(time));
        }

        output!(self.log, "Done");
        latch_pin.clear_pwm()?;

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
