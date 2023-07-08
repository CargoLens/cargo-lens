use cargo_metadata::diagnostic::{Diagnostic, DiagnosticLevel};
use cargo_metadata::Message;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankedDiagnostic {
    Warn(Diagnostic),
    Error(Diagnostic),
    Ice(Diagnostic),
    FailureNote(Diagnostic),
    Note(Diagnostic),
    Help(Diagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CargoState {
    Nothing,
    Warnings,
    Errors,
}

impl From<&Vec<RankedDiagnostic>> for CargoState {
    fn from(values: &Vec<RankedDiagnostic>) -> Self {
        if values.is_empty() {
            return Self::Nothing;
        }
        let mut has_warns = false;
        values
            .iter()
            .find_map(|val| match val {
                RankedDiagnostic::Error(_) => Some(Self::Errors),
                RankedDiagnostic::Warn(_) => {
                    has_warns = true;
                    None
                }
                _ => None,
            })
            .unwrap_or(if has_warns {
                Self::Warnings
            } else {
                Self::Nothing
            })
    }
}
/// For traits that you wish to implement with cargo, such as [DiagnosticImport]
pub struct CargoActor;

#[cfg_attr(test, mockall::automock(type Error=();))]
pub trait CargoImport: Sized {
    type Error: std::fmt::Debug + Sized;
    fn fetch() -> Result<Vec<RankedDiagnostic>, Self::Error>;
}

impl CargoImport for CargoActor {
    type Error = std::io::Error;
    fn fetch() -> Result<Vec<RankedDiagnostic>, <Self as CargoImport>::Error> {
        let args = vec!["check", "--message-format=json"];
        #[cfg(feature = "debug_socket")]
        let args = {
            // keep the vector immutable when not doing this.
            let mut res = args.clone();
            res.push("-F debug_socket");
            res
        };

        let mut command = Command::new("cargo")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let reader = std::io::BufReader::new(command.stdout.take().unwrap());

        let stream = cargo_metadata::Message::parse_stream(reader);
        let mut res = vec![];
        for msg in stream {
            match msg {
                Ok(Message::CompilerMessage(good_msg)) => res.push(good_msg.message.into()),
                Ok(Message::BuildFinished(_)) => break,
                _ => continue,
            }
        }

        let _output = command.wait()?;
        Ok(res)
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
