use std::collections::BTreeMap;

use evntwire_config::{Action, Binding};

use crate::Event;

/// Returns the action from the last binding whose event and conditions match.
pub fn matching_action<'a>(
    bindings: &'a [Binding],
    event: &Event<'_>,
    states: &BTreeMap<String, bool>,
) -> Option<&'a Action> {
    bindings
        .iter()
        .rev()
        .find(|binding| {
            binding.source == event.source
                && binding.button == event.button
                && binding.event == event.kind
                && binding
                    .when
                    .iter()
                    .all(|(name, value)| states.get(name) == Some(value))
        })
        .map(|binding| &binding.action)
}

#[cfg(test)]
#[path = "matching_binding.tests.rs"]
mod tests;
