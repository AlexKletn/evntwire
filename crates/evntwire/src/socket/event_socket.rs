use std::{
    collections::BTreeSet,
    fs, io,
    os::{
        fd::{AsFd, BorrowedFd},
        unix::{fs::MetadataExt, net::UnixDatagram},
    },
    path::{Path, PathBuf},
    time::Instant,
};

use evntwire_config::Action;

use crate::Router;

use super::message::Message;

pub struct EventSocket {
    socket: UnixDatagram,
    path: PathBuf,
    identity: (u64, u64),
    registered: BTreeSet<(String, String)>,
}

impl EventSocket {
    pub fn bind(path: &Path) -> io::Result<Self> {
        let parent = path
            .parent()
            .filter(|_| path.is_absolute())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Event socket requires an absolute path",
                )
            })?;
        if fs::metadata(parent)?.mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Event socket requires a private directory",
            ));
        }
        let socket = UnixDatagram::bind(path)?;
        let metadata = fs::symlink_metadata(path)?;
        let input = Self {
            socket,
            path: path.to_owned(),
            identity: (metadata.dev(), metadata.ino()),
            registered: BTreeSet::new(),
        };
        input.socket.set_nonblocking(true)?;
        Ok(input)
    }

    pub fn receive<'a>(
        &mut self,
        router: &mut Router<'a>,
        now: Instant,
    ) -> io::Result<Vec<&'a Action>> {
        let mut packet = [0; Message::MAX_BYTES + 1];
        let (length, sender) = self.socket.recv_from(&mut packet)?;
        match Message::parse(&packet[..length])? {
            Message::Register { source, button } => {
                if !self
                    .registered
                    .insert((source.to_owned(), button.to_owned()))
                {
                    return Err(io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        "Button is already registered",
                    ));
                }
            }
            Message::Event(event) => {
                if !self
                    .registered
                    .contains(&(event.source.to_owned(), event.button.to_owned()))
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Button is not registered",
                    ));
                }
                return Ok(router.dispatch(&event, now));
            }
            Message::SetState { name, value } => router.set_state(name, value),
            Message::GetState { name } => {
                let response = match router.state(name) {
                    Some(true) => "true",
                    Some(false) => "false",
                    None => "unknown",
                };
                self.socket.send_to_addr(response.as_bytes(), &sender)?;
            }
        }
        Ok(Vec::new())
    }
}

impl AsFd for EventSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
}

impl Drop for EventSocket {
    fn drop(&mut self) {
        if fs::symlink_metadata(&self.path)
            .is_ok_and(|metadata| (metadata.dev(), metadata.ino()) == self.identity)
        {
            let _ = fs::remove_file(&self.path);
        }
    }
}

#[cfg(test)]
#[path = "event_socket.tests.rs"]
mod tests;
