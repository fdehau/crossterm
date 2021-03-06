//! A module which contains the commands that can be used for windows systems.

use super::{IAlternateScreenCommand, IEnableAnsiCommand, TerminalOutput};

use kernel::windows_kernel::{ansi_support, ConsoleMode, Handle, ScreenBuffer};
use winapi::shared::minwindef::DWORD;
use winapi::um::wincon;
use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

use std::io::Result;

/// This command is used for enabling and disabling ANSI code support for windows systems,
/// For more info check: https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences.
#[derive(Clone, Copy)]
pub struct EnableAnsiCommand {
    mask: DWORD,
}

impl EnableAnsiCommand {
    pub fn new() -> EnableAnsiCommand {
        let command = EnableAnsiCommand {
            mask: ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        };
        command
    }
}

impl IEnableAnsiCommand for EnableAnsiCommand {
    fn enable(&self) -> Result<bool> {
        // we need to check whether we tried to enable ansi before. If we have we can just return if that had succeeded.
        if ansi_support::has_been_tried_to_enable_ansi() && ansi_support::ansi_enabled() {
            return Ok(ansi_support::windows_supportable());
        } else {
            let console_mode = ConsoleMode::new()?;

            let mut dw_mode = console_mode.mode()?;

            dw_mode |= self.mask;

            console_mode.set_mode(dw_mode)?;
            Ok(true)
        }
    }

    fn disable(&self) -> Result<()> {
        if ansi_support::ansi_enabled() {
            let console_mode = ConsoleMode::new()?;

            let mut dw_mode = console_mode.mode()?;

            dw_mode &= !self.mask;

            console_mode.set_mode(dw_mode)?;

            ansi_support::set_ansi_enabled(false);
        }
        Ok(())
    }
}

/// This command is used for enabling and disabling raw mode for windows systems.
/// For more info check: https://docs.microsoft.com/en-us/windows/console/high-level-console-modes.
#[derive(Clone, Copy)]
pub struct RawModeCommand {
    mask: DWORD,
}
use self::wincon::{ENABLE_LINE_INPUT, ENABLE_WRAP_AT_EOL_OUTPUT};
impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand {
            mask: ENABLE_WRAP_AT_EOL_OUTPUT | ENABLE_LINE_INPUT,
        }
    }
}

impl RawModeCommand {
    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        let console_mode = ConsoleMode::new()?;

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode & !self.mask;

        console_mode.set_mode(new_mode)?;

        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&self) -> Result<()> {
        let console_mode = ConsoleMode::new()?;

        let dw_mode = console_mode.mode()?;

        let new_mode = dw_mode | self.mask;

        console_mode.set_mode(new_mode)?;

        return Ok(());
    }
}

/// This command is used for switching to alternate screen and back to main screen.
/// check https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes for more info
pub struct ToAlternateScreenCommand;

impl ToAlternateScreenCommand {
    pub fn new() -> ToAlternateScreenCommand {
        return ToAlternateScreenCommand {};
    }
}

impl IAlternateScreenCommand for ToAlternateScreenCommand {
    fn enable(&self, _stdout: &mut TerminalOutput) -> Result<()> {
        let alternate_screen = ScreenBuffer::create();
        alternate_screen.show()?;
        Ok(())
    }

    fn disable(&self, _stdout: &TerminalOutput) -> Result<()> {
        let screen_buffer = ScreenBuffer::from(Handle::output_handle()?);
        screen_buffer.show()?;
        Ok(())
    }
}
