// Клиент, отправляющий серверу число

mod config;
use config::Config;

#[allow(unused_imports)]
use std::net::{TcpStream, SocketAddr, IpAddr};

use std::io::Write;

// Библиотека дает интерфейс к системным сокетам
use socket2::{Socket, Domain, Type, Protocol}; 

fn main() -> std::io::Result<()> {
    let conf: Config = argh::from_env();
    let ip = conf.host.parse::<IpAddr>()
        .expect("Не удалось распознать IP-адрес");
    let sock_addr = SocketAddr::new(ip, conf.port);

    // Удобно, однако слишком просто
    /* let mut stream = TcpStream::connect("127.0.0.1:8080")?; */

    // Используем более гибкий системный интерфейс.
    // Создаем сокет в домене IPv4, протокол -- TCP
    let socket = Socket::new(
        Domain::ipv4(), Type::stream(), Some(Protocol::tcp())
    )?;

    // Соединяемся с хостом
    socket.connect(&sock_addr.into())
                .expect("Не удалось сделать connect()!");

    // Превращаем сокет в поток, по аналогии с файлами. Примерно то же самое,
    // что файловый дескриптор в C
    let mut stream = socket.into_tcp_stream(); 

    // Отправляем серверу число в порядке байт big-endian (сетевой)
    stream.write(&conf.number.to_be_bytes()) 
        .expect("Не удалось отправить число!");

    println!("Число отправлено.");

    Ok(())
}
