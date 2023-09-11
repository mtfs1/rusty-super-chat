use std::io::Read;
use std::net::TcpListener;
use std::thread;


fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("0.0.0.0:1234")?;

    for conn in socket.incoming() {
        let mut conn = conn?;
        thread::spawn(move || {
            let mut buff = String::new();
            conn.read_to_string(&mut buff).unwrap();
            println!("{buff}");
        });
    }

    Ok(())
}

