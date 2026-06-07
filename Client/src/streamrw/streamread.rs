use std::net::TcpStream;
use std::io::Read;


pub fn read_from_stream(stream : &mut TcpStream) -> Option<String>{
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer);
    let msg;
    let response;
    match n {
        Ok(0) => {
            return None;
        }
        Ok(n) => {
            msg = std::str::from_utf8(&buffer[..n]);
            match msg {
                Ok(mut msg) => {
                    msg = msg.trim();
                    response = msg.to_string();
                    return Some(response);
                }
                Err(_) => {
                    return None;
                }
            }
        }
        Err(_) => {
            return None;
        }
    }
    
}