// Клиент, отправляющий серверу число

#[allow(unused_imports)]
use std::net::TcpStream;

use std::io::Write;
use std::net::SocketAddr;

// Библиотека дает интерфейс к системным сокетам
use socket2::{Socket, Domain, Type, Protocol}; 

fn main() -> std::io::Result<()> {
    let n = std::env::args() // берем переданные программе аргументы
                    .nth(1) // берем второй, поскольку первый -- имя программы
                    // Этот и дальнейшие вызовы .expect() -- обработка ошибок
                    .expect("Введите число, которе надо передать серверу!");

    let n = i32::from_str_radix(&n, 10) // переводим строку в число
                    .expect("Введите другое число!");

    // Удобно, однако слишком просто
    /* let mut stream = TcpStream::connect("127.0.0.1:8080")?; */

    // Используем более гибкий системный интерфейс.
    // Создаем сокет в домене IPv4, протокол -- TCP
    let socket = Socket::new(
        Domain::ipv4(), Type::stream(), Some(Protocol::tcp())
    )?;

    // Соединяемся с хостом
    socket.connect(&"127.0.0.1:8080".parse::<SocketAddr>() 
                    .expect("Неправильно указан IP-адрес")
                    .into())
                .expect("Не удалось сделать connect()!");

    // Превращаем сокет в поток, по аналогии с файлами. Примерно то же самое,
    // что файловый дескриптор в C
    let mut stream = socket.into_tcp_stream(); 

    // Отправляем серверу число в порядке байт big-endian (сетевой)
    stream.write(&n.to_be_bytes()) 
        .expect("Не удалось отправить число!");

    println!("Число отправлено.");

    Ok(())
}
