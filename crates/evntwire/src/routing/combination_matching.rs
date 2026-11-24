use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

use evntwire_config::Combination;

/// Select the longest matching combination containing the newly pressed button.
/// Equal-length matches follow configuration order.
pub fn matching_combination<'a>(
    combinations: &'a [Combination],
    button: (&str, &str),
    held: &BTreeMap<(&str, &str), Instant>,
    states: &BTreeMap<String, bool>,
    now: Instant,
) -> Option<&'a Combination> {
    combinations
        .iter()
        .rev()
        .filter(|candidate| {
            contains_button(candidate, button)
                && candidate
                    .when
                    .iter()
                    .all(|(name, value)| states.get(name) == Some(value))
                && candidate.buttons.iter().all(|member| {
                    held.get(&(member.source.as_str(), member.button.as_str()))
                        .and_then(|pressed| now.checked_duration_since(*pressed))
                        .is_some_and(|elapsed| {
                            elapsed <= Duration::from_millis(candidate.window_ms)
                        })
                })
        })
        .max_by_key(|candidate| candidate.buttons.len())
}

pub fn combination_window_ms(combinations: &[Combination], button: (&str, &str)) -> u64 {
    combinations
        .iter()
        .filter(|candidate| candidate.buttons.len() > 1 && contains_button(candidate, button))
        .map(|candidate| candidate.window_ms)
        .fold(Combination::DEFAULT_WINDOW_MS, u64::max)
}

fn contains_button(combination: &Combination, button: (&str, &str)) -> bool {
    combination
        .buttons
        .iter()
        .any(|member| (member.source.as_str(), member.button.as_str()) == button)
}

#[cfg(test)]
#[path = "combination_matching.tests.rs"]
mod tests;
