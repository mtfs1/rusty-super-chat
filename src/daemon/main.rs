use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex, Arc};
use std::thread;


fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("0.0.0.0:1234")?;

    let rooms = Arc::new(
        Mutex::new(HashMap::<String, Vec<TcpStream>>::new())
    );

    for conn in socket.incoming() {
        let mut conn = conn?;
        let rooms = rooms.clone();

        thread::spawn(move || {
            let conn_clone = conn.try_clone().unwrap();

            let mut buff = String::new();
            let mut reader = BufReader::new(&mut conn);
            let bytes_read = reader
                .fill_buf()
                .unwrap()
                .read_line(&mut buff)
                .unwrap();
            reader.consume(bytes_read);

            let mut iter = buff.trim().split("&");
            let name = match iter.next() {
                Some(val) => val.to_string(),
                None => {
                    println!("[INFO] Connection failed, no name \
                        informed for user");
                    return;
                }
            };
            let room = match iter.next() {
                Some(val) => val.to_string(),
                None => {
                    println!("[INFO] Connection failed, no room \
                        informed for user");
                    return;
                }
            };

            {
                let mut rooms = rooms.lock().unwrap();
                let room = {
                    if let None = rooms.get(&room) {
                        rooms.insert(room.clone(), Vec::new());
                    }
                    rooms.get_mut(&room).unwrap()
                };
                room.push(conn_clone);
            }

            println!("[INFO] User {name} succesfully entered \
                room {room}");

            loop {
                buff.clear();
                let bytes_read = reader.read_line(&mut buff).unwrap();
                reader.consume(bytes_read);

                if bytes_read == 0 {
                    break;
                }

                {
                    let mut rooms = rooms.lock().unwrap();
                    let room = rooms.get_mut(&room).unwrap();

                    for conn in room {
                        conn.write_all(format!("{name}: {buff}")
                            .as_bytes())
                            .unwrap();
                    }
                }
            }
        });
    }

    Ok(())
}

