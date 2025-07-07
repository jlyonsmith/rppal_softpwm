# RPPAL Software PWM CLI tool

[![coverage](https://shields.io/endpoint?url=https://raw.githubusercontent.com/jlyonsmith/rppal_softpwm/main/coverage.json)](https://github.com/jlyonsmith/rppal_softpwm/blob/main/coverage.json)
[![Crates.io](https://img.shields.io/crates/v/rppal_softpwm.svg)](https://crates.io/crates/rppal_softpwm)
[![Docs.rs](https://docs.rs/rppal_softpwm/badge.svg)](https://docs.rs/rppal_softpwm)

This is a crate that provides a command-line interface for controlling software PWM on Raspberry Pi GPIO pins.

You can provide a sequence of angles and durations to control the PWM signal. The angles represent the duty cycle of the PWM signal, and the durations represent the time for which the signal should be on or off. See `softpwm --help` for more information.
