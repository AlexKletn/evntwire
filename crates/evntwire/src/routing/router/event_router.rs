use std::{
    collections::{BTreeMap, BTreeSet},
    time::{Duration, Instant},
};

use evntwire_config::{Action, Combination, Config, RepeatPolicy};

use crate::{
    Event, GestureTracker, GestureTrigger, RepeatTracker, combination_window_ms, matching_action,
    matching_combination,
};

use super::button_state::ButtonState;

pub struct Router<'a> {
    config: &'a Config,
    states: BTreeMap<String, bool>,
    buttons: BTreeMap<(String, String), ButtonState<'a>>,
}

impl<'a> Router<'a> {
    pub fn new(config: &'a Config) -> Result<Self, &'static str> {
        for combination in &config.combinations {
            let unique = combination
                .buttons
                .iter()
                .map(|button| (&button.source, &button.button))
                .collect::<BTreeSet<_>>();
            if unique.is_empty()
                || unique.len() != combination.buttons.len()
                || combination.window_ms == 0
            {
                return Err(
                    "Each combination requires unique buttons and a positive timing window",
                );
            }
        }
        let mut buttons = BTreeMap::<_, ButtonState<'a>>::new();
        for gesture in &config.gestures {
            let button = buttons
                .entry((gesture.source.clone(), gesture.button.clone()))
                .or_default();
            if button.gesture.is_some() || gesture.long_ms == 0 || gesture.double_ms == 0 {
                return Err("Each button allows one gesture with positive timing windows");
            }
            button.gesture = Some((gesture, GestureTracker::new(gesture)));
        }
        for repeat in &config.repeats {
            let button = buttons
                .entry((repeat.source.clone(), repeat.button.clone()))
                .or_default();
            if button.repeat.is_some() {
                return Err("Each button allows one repeat policy");
            }
            if matches!(repeat.policy, RepeatPolicy::Timer { .. })
                && !config.bindings.iter().any(|binding| {
                    binding.source == repeat.source
                        && binding.button == repeat.button
                        && binding.event == "repeat"
                })
            {
                return Err("Repeat timer requires a repeat binding");
            }
            button.repeat = Some(RepeatTracker::new(&repeat.policy)?);
        }
        Ok(Self {
            config,
            states: BTreeMap::new(),
            buttons,
        })
    }

    pub fn set_state(&mut self, name: &str, value: Option<bool>) {
        if let Some(value) = value {
            self.states.insert(name.into(), value);
        } else {
            self.states.remove(name);
        }
    }

    pub fn state(&self, name: &str) -> Option<bool> {
        self.states.get(name).copied()
    }

    /// Dispatch events at the current time before advancing timers due at that time.
    pub fn dispatch(&mut self, event: &Event<'_>, now: Instant) -> Vec<&'a Action> {
        let id = (event.source.to_owned(), event.button.to_owned());
        match event.kind {
            "press" => self.press(id, now),
            "release" => self.release(&id, now),
            "repeat" => self
                .buttons
                .get(&id)
                .filter(|button| {
                    button.held_since.is_some()
                        && !button.consumed
                        && button.pending.is_none()
                        && button.gesture.is_none()
                        && button
                            .repeat
                            .as_ref()
                            .is_none_or(RepeatTracker::accepts_native_repeat)
                })
                .and_then(|_| matching_action(&self.config.bindings, event, &self.states))
                .into_iter()
                .collect(),
            _ => matching_action(&self.config.bindings, event, &self.states)
                .into_iter()
                .collect(),
        }
    }

    pub fn cancel_button(&mut self, source: &str, button: &str) {
        let id = (source.to_owned(), button.to_owned());
        if let Some(state) = self.buttons.get_mut(&id) {
            state.consume();
            state.consumed = false;
            state.held_since = None;
            if state.gesture.is_none() && state.repeat.is_none() {
                self.buttons.remove(&id);
            }
        }
    }

    pub fn deadline(&self) -> Option<Instant> {
        self.buttons
            .values()
            .filter_map(ButtonState::deadline)
            .min()
    }

    pub fn advance(&mut self, now: Instant) -> Vec<&'a Action> {
        let mut actions = Vec::new();
        for (id, button) in &mut self.buttons {
            let event = (id.0.as_str(), id.1.as_str());
            if let Some((gesture, tracker)) = &mut button.gesture {
                actions.extend(gesture_action(
                    self.config,
                    &self.states,
                    event,
                    gesture,
                    tracker.advance(now),
                ));
            }
            if button.pending.is_some_and(|deadline| deadline <= now) {
                actions.extend(flush_press(self.config, &self.states, event, button));
            }
            if button
                .repeat
                .as_mut()
                .is_some_and(|repeat| repeat.advance(now))
            {
                actions.extend(binding_action(self.config, &self.states, event, "repeat"));
            }
        }
        actions
    }

    fn press(&mut self, id: (String, String), now: Instant) -> Vec<&'a Action> {
        let button = self.buttons.entry(id.clone()).or_default();
        if button.held_since.is_some() {
            return Vec::new();
        }
        button.held_since = Some(now);
        let event = (id.0.as_str(), id.1.as_str());
        let held = self
            .buttons
            .iter()
            .filter_map(|((source, button), state)| {
                state
                    .held_since
                    .filter(|_| !state.consumed)
                    .map(|time| ((source.as_str(), button.as_str()), time))
            })
            .collect::<BTreeMap<_, _>>();
        let combination =
            matching_combination(&self.config.combinations, event, &held, &self.states, now)
                .filter(|candidate| candidate.buttons.len() > 1);
        let default_window = Duration::from_millis(Combination::DEFAULT_WINDOW_MS);
        let mut simultaneous = held
            .iter()
            .filter(|(_, pressed)| {
                now.checked_duration_since(**pressed)
                    .is_some_and(|elapsed| elapsed <= default_window)
            })
            .map(|(&(source, button), _)| (source.to_owned(), button.to_owned()))
            .collect::<Vec<_>>();
        if let Some(combination) = combination {
            simultaneous.extend(
                combination
                    .buttons
                    .iter()
                    .map(|member| (member.source.clone(), member.button.clone())),
            );
        }
        if simultaneous.len() > 1 {
            for member in simultaneous {
                if let Some(state) = self.buttons.get_mut(&member) {
                    state.consume();
                }
            }
            return combination
                .map(|candidate| &candidate.action)
                .into_iter()
                .collect();
        }

        let window = combination_window_ms(&self.config.combinations, event);
        let button = self
            .buttons
            .get_mut(&id)
            .expect("pressed button is registered");
        if let Some((gesture, tracker)) = &mut button.gesture {
            return gesture_action(
                self.config,
                &self.states,
                event,
                gesture,
                tracker.press(now, window),
            )
            .into_iter()
            .collect();
        }
        button.pending = now.checked_add(Duration::from_millis(window));
        if button.pending.is_none() {
            return flush_press(self.config, &self.states, event, button)
                .into_iter()
                .collect();
        }
        Vec::new()
    }

    fn release(&mut self, id: &(String, String), now: Instant) -> Vec<&'a Action> {
        let Some(button) = self.buttons.get_mut(id) else {
            return Vec::new();
        };
        if button.held_since.is_none() {
            return Vec::new();
        }
        let event = (id.0.as_str(), id.1.as_str());
        let mut actions = Vec::new();
        if let Some((gesture, tracker)) = &mut button.gesture {
            actions.extend(gesture_action(
                self.config,
                &self.states,
                event,
                gesture,
                tracker.release(now),
            ));
        } else if !button.consumed {
            if button.pending.is_some() {
                actions.extend(flush_press(self.config, &self.states, event, button));
            }
            actions.extend(binding_action(self.config, &self.states, event, "release"));
        }
        button.held_since = None;
        button.consumed = false;
        if let Some(repeat) = &mut button.repeat {
            repeat.cancel();
        }
        if button.gesture.is_none() && button.repeat.is_none() {
            self.buttons.remove(id);
        }
        actions
    }
}

fn flush_press<'a>(
    config: &'a Config,
    states: &BTreeMap<String, bool>,
    id: (&str, &str),
    button: &mut ButtonState<'_>,
) -> Option<&'a Action> {
    button.pending = None;
    if let (Some(repeat), Some(pressed)) = (&mut button.repeat, button.held_since) {
        repeat.arm(pressed);
    }
    single_action(config, states, id)
}

fn single_action<'a>(
    config: &'a Config,
    states: &BTreeMap<String, bool>,
    id: (&str, &str),
) -> Option<&'a Action> {
    config.combinations.iter().find(|candidate| {
        matches!(candidate.buttons.as_slice(), [button] if (button.source.as_str(), button.button.as_str()) == id)
            && candidate.when.iter().all(|(name, value)| states.get(name) == Some(value))
    }).map(|candidate| &candidate.action).or_else(|| binding_action(config, states, id, "press"))
}

fn gesture_action<'a>(
    config: &'a Config,
    states: &BTreeMap<String, bool>,
    id: (&str, &str),
    gesture: &'a evntwire_config::Gesture,
    trigger: Option<GestureTrigger>,
) -> Option<&'a Action> {
    match trigger? {
        GestureTrigger::Single => single_action(config, states, id),
        GestureTrigger::Double => Some(&gesture.double_action),
        GestureTrigger::Long => Some(&gesture.long_action),
    }
}

fn binding_action<'a>(
    config: &'a Config,
    states: &BTreeMap<String, bool>,
    (source, button): (&str, &str),
    kind: &str,
) -> Option<&'a Action> {
    matching_action(
        &config.bindings,
        &Event {
            source,
            button,
            kind,
        },
        states,
    )
}

#[cfg(test)]
#[path = "event_router.tests.rs"]
mod tests;
