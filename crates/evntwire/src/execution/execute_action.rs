use std::{
    io,
    process::{Child, Command, Stdio},
};

use evntwire_config::Action;

use super::KeyOutput;

/// The caller owns the returned child and must reap it after exit.
pub fn execute_action(action: &Action, keys: &mut impl KeyOutput) -> io::Result<Option<Child>> {
    match action {
        Action::Key { code } => {
            keys.tap(code)?;
            Ok(None)
        }
        Action::Command { program, args } => Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .spawn()
            .map(Some),
    }
}

#[cfg(test)]
#[path = "execute_action.tests.rs"]
mod tests;
