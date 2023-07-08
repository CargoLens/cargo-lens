use cargo_metadata::diagnostic::Diagnostic;
use cargo_metadata::Message;
use std::process::{Command, Stdio};

/// For traits that you wish to implement with cargo, such as [`DiagnosticImport`]
pub struct CargoActor;

#[cfg_attr(test, mockall::automock(type Error=();))]
pub trait CargoImport: Sized {
    type Error: std::fmt::Debug + Sized;
    fn fetch() -> Result<Vec<Diagnostic>, Self::Error>;
}

impl CargoImport for CargoActor {
    type Error = std::io::Error;
    fn fetch() -> Result<Vec<Diagnostic>, <Self as CargoImport>::Error> {
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
#[cfg(test)]
mod test {
    // TODO: Create a workspace member that will act as a test fixture., the test will set that ws-member as the directory, then invoke run, and will assert on the result
}
