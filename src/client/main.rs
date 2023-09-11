use std::io::{prelude::*, stdin, self};
use std::error::Error;
use std::net::TcpStream;


fn main() -> Result<(), Box<dyn Error>> {
    print_header();
    let mut server = connect_to_server()?;
    server.write("Hello World!".as_bytes())?;

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

