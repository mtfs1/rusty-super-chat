use std::collections::HashMap;
use std::io::Read;
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
            let mut buff = String::new();
            conn.read_to_string(&mut buff).unwrap();

            let mut iter = buff.split("&");
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
                room.push(conn.try_clone().unwrap());
            }

            println!("[INFO] User {name} succesfully entered \
                room {room}");
        });
    }

    Ok(())
}

