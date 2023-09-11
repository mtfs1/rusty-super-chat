use std::io::{prelude::*, stdin, self};
use std::error::Error;
use std::net::TcpStream;


fn main() -> Result<(), Box<dyn Error>> {
    print_header();
    let mut server = connect_to_server()?;

    register_user(&mut server)?;

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

