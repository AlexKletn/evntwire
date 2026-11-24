#[cfg(target_os = "linux")]
mod daemon;

#[cfg(target_os = "linux")]
fn main() -> std::process::ExitCode {
    match daemon::run(std::env::args_os().skip(1)) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("evntwire: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() -> std::process::ExitCode {
    eprintln!("evntwire requires Linux");
    std::process::ExitCode::FAILURE
}
