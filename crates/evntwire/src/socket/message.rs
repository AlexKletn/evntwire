use std::{io, str};

use crate::Event;

pub(super) enum Message<'a> {
    Register { source: &'a str, button: &'a str },
    Event(Event<'a>),
    SetState { name: &'a str, value: Option<bool> },
    GetState { name: &'a str },
}

impl<'a> Message<'a> {
    pub(super) const MAX_BYTES: usize = 1024;

    pub(super) fn parse(packet: &'a [u8]) -> io::Result<Self> {
        if packet.len() > Self::MAX_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Event datagram exceeds 1024 bytes",
            ));
        }
        let message = str::from_utf8(packet)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let mut fields = message.split('\t');
        match (
            fields.next(),
            fields.next(),
            fields.next(),
            fields.next(),
            fields.next(),
        ) {
            (Some("register"), Some(source), Some(button), None, None)
                if !source.trim().is_empty() && !button.trim().is_empty() =>
            {
                Ok(Self::Register { source, button })
            }
            (
                Some("event"),
                Some(source),
                Some(button),
                Some(kind @ ("press" | "release" | "repeat")),
                None,
            ) => Ok(Self::Event(Event {
                source,
                button,
                kind,
            })),
            (Some("state"), Some(operation), Some(name), value, None) if valid_state_name(name) => {
                match (operation, value) {
                    ("set", Some("true")) => Ok(Self::SetState {
                        name,
                        value: Some(true),
                    }),
                    ("set", Some("false")) => Ok(Self::SetState {
                        name,
                        value: Some(false),
                    }),
                    ("unset", None) => Ok(Self::SetState { name, value: None }),
                    ("get", None) => Ok(Self::GetState { name }),
                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid state operation",
                    )),
                }
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid event datagram",
            )),
        }
    }
}

fn valid_state_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 128
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
}
