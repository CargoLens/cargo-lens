use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use cargo_metadata::Message;
use std::process::{Command, Stdio};

pub enum RankedDiagnostic {
    Warn(Diagnostic),
    Error(Diagnostic),
    Ice(Diagnostic),
    FailureNote(Diagnostic),
    Note(Diagnostic),
    Help(Diagnostic),
}

pub trait DiagnosticImport: Sized {
    type Error: Sized;
    fn fetch() -> Result<Vec<Self>, Self::Error>;
}

impl DiagnosticImport for RankedDiagnostic {
    type Error = std::io::Error;
    fn fetch() -> Result<Vec<Self>, <RankedDiagnostic as DiagnosticImport>::Error> {
        let mut command = Command::new("cargo")
            .args(&["build", "--message-format=json"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let reader = std::io::BufReader::new(command.stdout.take().unwrap());

        let rus = cargo_metadata::Message::parse_stream(reader);
        let rus = rus.filter_map(|msg| match msg {
            Ok(Message::CompilerMessage(good_msg)) => Some(good_msg.message.into()),
            _ => None,
        });
        let _output = command.wait()?;
        Ok(rus.collect())
    }
}

impl From<Diagnostic> for RankedDiagnostic {
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

#[cfg(test)]
mod test {
    // TODO: Create a workspace member that will act as a test fixture., the test will set that ws-member as the directory, then invoke run, and will assert on the result
}
