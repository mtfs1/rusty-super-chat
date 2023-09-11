use std::io::{prelude::*, stdin, self, BufReader};
use std::error::Error;
use std::net::TcpStream;
use std::thread;


fn main() -> Result<(), Box<dyn Error>> {
    print_header();
    let mut server = connect_to_server()?;

    register_user(&mut server)?;

    {
        let mut server = server.try_clone().unwrap();
        thread::spawn(move || {
            let mut buff = String::new();
            let mut reader = BufReader::new(&mut server);

            loop {
                buff.clear();
                let bytes_read = reader
                    .fill_buf()
                    .unwrap()
                    .read_line(&mut buff)
                    .unwrap();
                reader.consume(bytes_read);
                buff.pop();

                println!("{buff}");
                print!("> ");
                io::stdout().flush().expect("Falha ao dar flush em buffer");
            }
        });
    }

    let mut buff = String::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Falha ao dar flush em buffer");

        buff.clear();
        stdin().read_line(&mut buff)?;

        if buff.trim() == "exit" {
            break;
        }

        server.write_all(buff.as_bytes()).unwrap();
    }

    Ok(())
}

fn print_header() {
    println!("[SUPER CHAT]");
    println!("Bem vindo ao super chat! \
        Digite o endereco do servidor que \
        deseja conectar-se:");
}

fn connect_to_server() -> Result<TcpStream, Box<dyn Error>> {
    loop {
        let server_addr = get_server_addr()?;
        let server = TcpStream::connect(server_addr);

        if let Err(_) = server {
            println!("O endereco digitado ou esta \
                    indisponivel ou nao existe");
            continue;
        }
        break Ok(server.unwrap());
    }
}

fn get_server_addr() -> Result<String, Box<dyn Error>> {
    print!("> ");
    io::stdout().flush().expect("Falha ao dar flush em buffer");

    let mut buff = String::new();
    stdin().lock().read_line(&mut buff)?;

    Ok(buff.trim().to_owned())
}

fn register_user(server: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Entre com o seu nome: ");
    print!("> ");
    io::stdout().flush().expect("Falha ao dar flush em buffer");

    let mut nome = String::new();
    stdin().read_line(&mut nome)?;
    nome.pop();

    println!("Entre com a sala que quer participar: ");
    print!("> ");
    io::stdout().flush().expect("Falha ao dar flush em buffer");

    let mut room = String::new();
    stdin().read_line(&mut room)?;
    room.pop();

    server.write(format!("{nome}&{room}").as_bytes())?;

    Ok(())
}

