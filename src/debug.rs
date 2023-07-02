use std::net::{TcpListener, TcpStream};
use std::os::unix::io::AsRawFd;
pub fn connect_to_iface() -> std::io::Result<TcpStream> {
    let listener = TcpListener::bind("localhost:8080")?;
    let stream = listener.accept()?;

    // redirect stderr to the socket
    let fd = stream.0.as_raw_fd();
    unsafe {
        libc::dup2(fd, libc::STDERR_FILENO);
    }

    Ok(stream.0)
}
