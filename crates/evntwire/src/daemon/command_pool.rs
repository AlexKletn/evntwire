use std::{io, process::Child};

use evntwire::{KeyOutput, execute_action};
use evntwire_config::Action;

#[derive(Default)]
pub(super) struct CommandPool {
    children: Vec<Child>,
}

impl CommandPool {
    const MAX_CHILDREN: usize = 16;

    pub(super) fn execute(&mut self, action: &Action, keys: &mut impl KeyOutput) -> io::Result<()> {
        if matches!(action, Action::Command { .. }) && self.children.len() >= Self::MAX_CHILDREN {
            return Err(io::Error::other("Concurrent command limit reached"));
        }
        if let Some(child) = execute_action(action, keys)? {
            self.children.push(child);
        }
        Ok(())
    }

    pub(super) fn reap(&mut self) {
        self.children.retain_mut(|child| match child.try_wait() {
            Ok(None) => true,
            Ok(Some(status)) => {
                if !status.success() {
                    eprintln!("evntwire: command {} exited with {status}", child.id());
                }
                false
            }
            Err(error) => {
                eprintln!("evntwire: cannot reap command {}: {error}", child.id());
                true
            }
        });
    }
}
