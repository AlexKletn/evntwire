use super::{Action, KeyOutput, execute_action, io};

#[derive(Default)]
struct RecordingOutput {
    taps: Vec<String>,
    unavailable: bool,
}

impl KeyOutput for RecordingOutput {
    fn tap(&mut self, code: &str) -> io::Result<()> {
        if self.unavailable {
            return Err(io::Error::from(io::ErrorKind::BrokenPipe));
        }
        self.taps.push(code.into());
        Ok(())
    }
}

#[test]
fn sends_a_single_tap_and_propagates_output_errors() {
    let mut keys = RecordingOutput::default();
    let action = Action::Key {
        code: "KEY_MUTE".into(),
    };

    assert!(execute_action(&action, &mut keys).unwrap().is_none());
    assert_eq!(keys.taps, ["KEY_MUTE"]);

    keys.unavailable = true;
    let error = execute_action(&action, &mut keys).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
}

#[test]
fn starts_a_program_with_literal_arguments_and_returns_its_child() {
    let mut keys = RecordingOutput::default();
    let action = Action::Command {
        program: "/usr/bin/test".into(),
        args: ["a b; c", "=", "a b; c"].map(String::from).into(),
    };

    let mut child = execute_action(&action, &mut keys).unwrap().unwrap();
    assert!(child.wait().unwrap().success());
    assert!(keys.taps.is_empty());
}

#[test]
fn reports_failure_to_start_a_program() {
    let mut keys = RecordingOutput::default();
    let action = Action::Command {
        program: String::new(),
        args: Vec::new(),
    };
    assert!(execute_action(&action, &mut keys).is_err());
}
