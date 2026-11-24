use std::io;

pub trait KeyOutput {
    fn tap(&mut self, code: &str) -> io::Result<()>;
}
