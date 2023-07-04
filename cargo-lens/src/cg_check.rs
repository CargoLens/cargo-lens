use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use cargo_metadata::Message;
use std::process::{Command, Stdio};

pub enum LensDiagnostic {
    Warn(Diagnostic),
    Error(Diagnostic),
    Ice(Diagnostic),
    FailureNote(Diagnostic),
    Note(Diagnostic),
    Help(Diagnostic),
}

impl From<Diagnostic> for LensDiagnostic {
    fn from(value: Diagnostic) -> Self {
        match value.level {
            DiagnosticLevel::Ice => Self::Ice(value),
            DiagnosticLevel::Error => Self::Error(value),
            DiagnosticLevel::Warning => Self::Warn(value),
            DiagnosticLevel::FailureNote => Self::FailureNote(value),
            DiagnosticLevel::Note => Self::Note(value),
            DiagnosticLevel::Help => Self::Help(value),
            _ => todo!("non-exhaustive"),
        }
    }
}

pub fn run() -> std::io::Result<Vec<Diagnostic>> {
    let mut command = Command::new("cargo")
        .args(&["build", "--message-format=json"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = std::io::BufReader::new(command.stdout.take().unwrap());

    let rus = cargo_metadata::Message::parse_stream(reader);
    let rus = rus.filter_map(|msg| match msg {
        Ok(Message::CompilerMessage(good_msg)) => Some(good_msg.message),
        _ => None,
    });
    let _output = command.wait()?;
    Ok(rus.collect())
}
