use std::os::fd::RawFd;

use termios::{
    tcsetattr, Termios, BRKINT, ECHO, ICANON, ICRNL, IGNBRK, IGNCR, INLCR, ISTRIP, IXON, PARMRK,
    TCSANOW,
};

use crate::error::Lc3Error;

/// Manages the terminal configuration used while running an LC-3 program.
///
/// The terminal configuration is changed when the guard is created and restored
/// to its original state when the guard is dropped.
///
/// This allows the terminal to be restored automatically when the emulator
/// finishes execution, including when execution returns an error.
pub struct TerminalGuard {
    fd: RawFd,
    original: Termios,
}

impl Drop for TerminalGuard {
    #[allow(clippy::unwrap_used)]
    fn drop(&mut self) {
        tcsetattr(self.fd, TCSANOW, &self.original).unwrap();
    }
}

impl TerminalGuard {
    /// Configures the terminal for LC-3 program execution.
    ///
    /// Canonical input mode and input echo are disabled so LC-3 programs can
    /// receive keyboard input without waiting for a newline or having the
    /// terminal echo the input automatically.
    ///
    /// The original terminal configuration is stored and restored when the
    /// returned guard is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the current terminal configuration cannot be read
    /// or the modified configuration cannot be applied.
    pub fn new(fd: RawFd) -> Result<Self, Lc3Error> {
        let original = Termios::from_fd(fd)?;

        let mut new_termios = original;
        new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(fd, TCSANOW, &new_termios)?;

        Ok(Self { fd, original })
    }
}
